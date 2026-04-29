use bytes::{Buf, BufMut, Bytes};
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
    pub fn deserialize<'p, Cmd>(
        &self,
    ) -> Result<Cmd::Response, ResponseError<<Cmd::Response as Response>::Error>>
    where
        Cmd: Command<'p>,
    {
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

        let mut all_fields = BytesSplit::new(output, b',');

        let Some(cmd) = all_fields.next() else {
            return Err(DecoderError::Malformed);
        };

        let raw_values = all_fields.collect::<Vec<_>>();

        Ok(Some(RawResponse { cmd, raw_values }))
    }
}

struct BytesSplit(Bytes, u8);

impl BytesSplit {
    fn new(inner: Bytes, split_at: u8) -> Self {
        Self(inner, split_at)
    }
}

impl Iterator for BytesSplit {
    type Item = Bytes;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        // find the index of the first delimiter
        let Some(i) = self
            .0
            .iter()
            .enumerate()
            .find_map(|(i, b)| (*b == self.1).then_some(i))
        else {
            // we're on the last element
            let last_elem = self.0.clone();
            self.0.clear();
            return Some(last_elem);
        };

        // extract the element
        let elem = self.0.split_to(i);
        // remove the comma
        self.0.advance(1);

        Some(elem)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use bytes::Bytes;

    #[test]
    fn split_works() {
        let bytes = Bytes::from(b"foo,bar,baz".as_slice());
        let mut split = BytesSplit::new(bytes, b',');

        assert_eq!(split.next().unwrap().as_ref(), b"foo");
        assert_eq!(split.next().unwrap().as_ref(), b"bar");
        assert_eq!(split.next().unwrap().as_ref(), b"baz");
        assert_eq!(split.next(), None);
    }
}
