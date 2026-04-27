pub mod command;

use bytes::BufMut;
use tokio_util::codec::{AnyDelimiterCodec, Decoder, Encoder};

use command::Command;

#[derive(Clone, Debug)]
pub struct Codec {
    // encoder: AnyDelimiterCodec,
    decoder: AnyDelimiterCodec,
}

impl Codec {
    pub fn new() -> Self {
        let decoder = AnyDelimiterCodec::new(b"\r".to_vec(), b"\r".to_vec());
        Self {
            // encoder: codec.clone(),
            decoder,
        }
    }
}

impl<T> Encoder<T> for Codec
where
    T: Command,
{
    type Error = std::io::Error;

    fn encode(
        &mut self,
        item: T,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        dst.extend_from_slice(item.as_bytes());

        for param in item.params() {
            dst.put_u8(b',');
            param.write_bytes(dst);
        }

        dst.put_u8(b'\r');

        Ok(())
    }
}

impl Decoder for Codec {
    type Item = String;
    type Error = <AnyDelimiterCodec as Decoder>::Error;

    fn decode(
        &mut self,
        src: &mut tokio_util::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let output = self.decoder.decode(src)?;
        let s = output.map(|out| String::from_utf8_lossy(out.as_ref()).into_owned());
        Ok(s)
    }
}
