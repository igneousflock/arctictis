use tokio_util::{
    bytes::{BufMut, Bytes, BytesMut},
    codec::{AnyDelimiterCodec, AnyDelimiterCodecError, Decoder, Encoder},
};

use crate::{
    bytes_split::BytesSplit,
    command::{Command, Params, Response},
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
        let est_len = Cmd::TEXT.len() + params.size_hint() + 1;
        dst.reserve(est_len);

        dst.extend_from_slice(Cmd::TEXT);
        for param in params {
            dst.put_u8(PARAM_DELIMITER);
            dst.extend_from_slice(param.as_ref());
        }

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

        let response = Cmd::Response::deserialize(self.raw_values.iter())?;

        Ok(response)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecoderError {
    #[error("malformed response")]
    Malformed,

    #[error("command must be executed in program mode")]
    NotAcceptable,

    #[error("command format error or invalid value")]
    ErrorResponse,

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

        if raw_values.len() == 1 {
            match raw_values[0].as_ref() {
                b"NG" => return Err(DecoderError::NotAcceptable),
                b"ERR" => return Err(DecoderError::ErrorResponse),
                _ => {}
            }
        }

        Ok(Some(RawResponse { cmd, raw_values }))
    }
}
