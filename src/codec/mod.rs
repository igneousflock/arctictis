mod command;

use bytes::BufMut;
use tokio_util::codec::{AnyDelimiterCodec, Decoder, Encoder};

pub use command::Command;

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

impl Encoder<Command> for Codec {
    type Error = std::io::Error;

    fn encode(
        &mut self,
        item: Command,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        dst.reserve(4); // 3 chars for command, one for return code

        let cmd: &'static str = item.into();
        dst.extend_from_slice(cmd.as_bytes());
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
