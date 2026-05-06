use bytes::{BufMut, Bytes};
use tokio_util::codec::{AnyDelimiterCodec, AnyDelimiterCodecError, Decoder, Encoder};

use crate::{
    bytes_split::BytesSplit,
    command::{Command, ParamBuffer, Params, Response},
};

const RETURN_CODE: u8 = b'\r';
const PARAM_DELIMITER: u8 = b',';

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

    fn encode(
        &mut self,
        item: Cmd,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let params = item.params();
        let est_len = Cmd::TEXT.len() + params.count() + params.total_size() + 1;
        dst.reserve(est_len);

        dst.extend_from_slice(Cmd::TEXT);
        params.serialize_to(ParamBuffer::new(dst));

        dst.put_u8(b'\r');

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

    fn decode(
        &mut self,
        src: &mut tokio_util::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
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
