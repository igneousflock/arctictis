use bytes::BufMut;
use tokio_util::codec::{AnyDelimiterCodec, AnyDelimiterCodecError, Decoder, Encoder};

use crate::command::{Command, ParamSet, Response};

#[derive(Clone, Debug)]
pub struct Codec {
    decoder: AnyDelimiterCodec,
}

impl Codec {
    pub fn new() -> Self {
        Self {
            decoder: AnyDelimiterCodec::new(b"\r".to_vec(), b"\r".to_vec()),
        }
    }
}

impl<'p, Cmd> Encoder<Cmd> for Codec
where
    Cmd: Command<'p>,
{
    type Error = std::io::Error;

    fn encode(
        &mut self,
        item: Cmd,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let params = item.param_set();
        let est_len = Cmd::TEXT.len() + params.count() + params.size() + 1;
        dst.reserve(est_len);

        dst.extend_from_slice(Cmd::TEXT);

        for param in item.param_set() {
            dst.put_u8(b',');
            param.write_bytes(dst);
        }

        dst.put_u8(b'\r');

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ResponseError<E> {
    #[error("response is for wrong command")]
    WrongCommand,

    #[error(transparent)]
    InvalidFields(#[from] E),
}

pub struct RawResponse {
    cmd: Vec<u8>,
    raw_values: Vec<u8>,
}

impl RawResponse {
    pub fn parse<'p, Cmd>(
        &self,
    ) -> Result<Cmd::Response, ResponseError<<Cmd::Response as Response>::Error>>
    where
        Cmd: Command<'p>,
    {
        if self.cmd != Cmd::TEXT {
            return Err(ResponseError::WrongCommand);
        }

        let response = Cmd::Response::parse_from_values(self.raw_values.split(|b| *b == b','))?;

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

        let mut fields = output.split(|b| *b == b',');
        let Some(cmd) = fields.next() else {
            return Err(DecoderError::Malformed);
        };

        Ok(Some(RawResponse {
            cmd: cmd.to_owned(),
            raw_values: output[4..].to_owned(),
        }))
    }
}
