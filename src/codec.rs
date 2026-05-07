use tokio_util::{
    bytes::{BufMut, Bytes, BytesMut},
    codec::{AnyDelimiterCodec, AnyDelimiterCodecError, Decoder, Encoder},
};

use crate::{
    bytes_split::BytesSplit,
    command::{Command, ParamBuffer, Params, Response},
};

pub(crate) const RETURN_CODE: u8 = b'\r';
pub(crate) const PARAM_DELIMITER: u8 = b',';

#[derive(Clone, Debug)]
pub struct Codec {
    decoder: AnyDelimiterCodec,
}

impl Codec {
    pub fn new() -> Self {
        Self {
            decoder: AnyDelimiterCodec::new(vec![RETURN_CODE], vec![RETURN_CODE]),
        }
    }
}

impl<Cmd> Encoder<Cmd> for Codec
where
    Cmd: Command,
{
    type Error = std::io::Error;

    fn encode(&mut self, item: Cmd, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let params = item.params();
        let est_len = Cmd::TEXT.len() + params.count() + params.total_size() + 1;
        dst.reserve(est_len);

        dst.extend_from_slice(Cmd::TEXT);
        params.serialize_to(ParamBuffer::new(dst));

        dst.put_u8(RETURN_CODE);

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ResponseError<E> {
    #[error("response is for wrong command")]
    WrongCommand,

    #[error("unexpected number of fields")]
    WrongNumberOfFields,

    #[error(transparent)]
    InvalidFields(#[from] E),
}

#[derive(Clone, Debug)]
pub struct RawResponse {
    cmd: Bytes,
    raw_values: Vec<Bytes>,
}

impl RawResponse {
    pub fn deserialize<Cmd: Command>(
        &self,
    ) -> Result<Cmd::Response, ResponseError<<Cmd::Response as Response>::Error>> {
        if self.cmd != Cmd::TEXT {
            return Err(ResponseError::WrongCommand);
        }
        if self.raw_values.len() != Cmd::Response::expected_field_count() {
            return Err(ResponseError::WrongNumberOfFields);
        }

        let response = Cmd::Response::deserialize(&self.raw_values)?;

        Ok(response)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecoderError {
    #[error("malformed response")]
    Malformed,

    #[error(transparent)]
    DelimiterError(#[from] AnyDelimiterCodecError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Decoder for Codec {
    type Item = RawResponse;
    type Error = DecoderError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // internal `AnyDelimiterCodec` is responsible for managing the buffer per the semantics of
        // `Decoder`
        let Some(output) = self.decoder.decode(src)? else {
            return Ok(None);
        };

        let mut all_fields = BytesSplit::new(output, PARAM_DELIMITER);

        let Some(cmd) = all_fields.next() else {
            return Err(DecoderError::Malformed);
        };

        let raw_values = all_fields.collect::<Vec<_>>();

        Ok(Some(RawResponse { cmd, raw_values }))
    }
}

#[cfg(test)]
mod tests {
    use claims::{assert_matches, assert_none, assert_ok};
    use tokio_util::{
        bytes::{Bytes, BytesMut},
        codec::{AnyDelimiterCodec, AnyDelimiterCodecError, Decoder, Encoder},
    };

    use crate::{
        Command, DecoderError, OkResponseError, ResponseError,
        codec::{Codec, RETURN_CODE, RawResponse},
        command::{OkResponse, command, range_param, range_response},
    };

    #[derive(Debug, thiserror::Error)]
    pub enum ParamError {
        #[error("invalid UTF-8 bytes")]
        Utf8Error(#[from] std::str::Utf8Error),
        #[error(transparent)]
        Parse(#[from] std::num::ParseIntError),
        #[error("invalid value, got `{0}`")]
        Invalid(u8),
    }

    range_param!(RangeParam(0..=15): u8);
    range_response!(RangeParam => ParamError : Invalid);

    command!(b"CMD": SimpleCommand);
    command!(b"CMD": ResponseCommand => RangeParam);
    command!(b"CMD": ParamCommand(RangeParam) => OkResponse);

    mod encode {
        use super::*;

        #[test]
        fn simple_command() {
            let buf = encode(SimpleCommand);

            assert_eq!(buf.as_ref(), b"CMD\r");
        }

        #[test]
        fn single_param() {
            let buf = encode(ParamCommand(RangeParam::new(0).unwrap()));

            assert_eq!(buf.as_ref(), b"CMD,0\r");
        }

        fn encode<C: Command>(cmd: C) -> BytesMut {
            let mut buf = BytesMut::new();
            Codec::new().encode(cmd, &mut buf).unwrap();
            buf
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn ok_response() {
            let item = decode(b"CMD,OK\r").unwrap().unwrap();

            assert_eq!(item.cmd.as_ref(), b"CMD");
            assert_eq!(item.raw_values, [Bytes::from(b"OK".as_slice())]);
        }

        #[test]
        fn multi_param_response() {
            let item = decode(b"CMD,1,2,3\r").unwrap().unwrap();

            assert_eq!(item.cmd.as_ref(), b"CMD");
            assert_eq!(
                item.raw_values,
                [
                    Bytes::from(b"1".as_slice()),
                    Bytes::from(b"2".as_slice()),
                    Bytes::from(b"3".as_slice())
                ],
            );
        }

        #[test]
        fn incomplete_response_returns_none() {
            let (response, result) = decode_buf(b"CMD,");

            assert_none!(assert_ok!(result));
            // buffer should not be changed
            assert_eq!(response.as_ref(), b"CMD,");
        }

        #[test]
        fn clears_buffer_on_complete_response() {
            let (response, result) = decode_buf(b"CMD,OK\r");

            assert_eq!(result.unwrap().unwrap().cmd.as_ref(), b"CMD");
            // buffer should be cleared
            assert!(response.is_empty());
        }

        #[test]
        fn partially_consumes_buffer_on_complete_response() {
            let (response, result) = decode_buf(b"CMD,OK\rCMD2");

            assert_eq!(result.unwrap().unwrap().cmd.as_ref(), b"CMD");
            // buffer should have single complete command removed
            assert_eq!(response.as_ref(), b"CMD2");
        }

        #[test]
        fn malformed_response() {
            let err = decode(b"\r").unwrap_err();

            assert_matches!(err, DecoderError::Malformed);
        }

        #[test]
        fn delimiter_error() {
            let mut codec = Codec::new();
            // reduce the max length of the internal delimiter decoder
            codec.decoder =
                AnyDelimiterCodec::new_with_max_length(vec![RETURN_CODE], vec![RETURN_CODE], 1);
            let mut response = BytesMut::from(b"CMD,OK\r".as_slice());

            let err = codec.decode(&mut response).unwrap_err();
            assert_matches!(
                err,
                DecoderError::DelimiterError(AnyDelimiterCodecError::MaxChunkLengthExceeded)
            );
        }

        fn decode(raw_response: &[u8]) -> Result<Option<RawResponse>, DecoderError> {
            decode_buf(raw_response).1
        }

        fn decode_buf(
            raw_response: &[u8],
        ) -> (BytesMut, Result<Option<RawResponse>, DecoderError>) {
            let mut raw_response = BytesMut::from(raw_response);
            let result = Codec::new().decode(&mut raw_response);

            (raw_response, result)
        }
    }

    mod deserialize {
        use super::*;

        #[test]
        fn ok_response() {
            decode(b"CMD,OK\r").deserialize::<SimpleCommand>().unwrap();
            // nothing to assert
        }

        #[test]
        fn single_param_response() {
            let response = decode(b"CMD,1\r").deserialize::<ResponseCommand>().unwrap();
            assert_eq!(response.value(), 1);
        }

        #[test]
        fn wrong_command() {
            let result = decode(b"FOO,OK\r")
                .deserialize::<SimpleCommand>()
                .unwrap_err();
            assert_matches!(result, ResponseError::WrongCommand);
        }

        #[test]
        fn wrong_number_of_fields() {
            let result = decode(b"CMD,OK,OK\r")
                .deserialize::<SimpleCommand>()
                .unwrap_err();
            assert_matches!(result, ResponseError::WrongNumberOfFields);
        }

        #[test]
        fn invalid_fields() {
            let result = decode(b"CMD,FOO\r")
                .deserialize::<SimpleCommand>()
                .unwrap_err();
            assert_matches!(
                result,
                ResponseError::InvalidFields(OkResponseError::UnexpectedValue)
            );
        }

        fn decode(raw: &[u8]) -> RawResponse {
            let mut raw = BytesMut::from(raw);
            Codec::new().decode(&mut raw).unwrap().unwrap()
        }
    }
}
