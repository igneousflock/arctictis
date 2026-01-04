use tokio_util::codec::{AnyDelimiterCodec, Decoder, Encoder};

#[derive(Clone, Debug)]
pub struct Codec {
    encoder: AnyDelimiterCodec,
    decoder: AnyDelimiterCodec,
}

impl Codec {
    pub fn new() -> Self {
        let codec = AnyDelimiterCodec::new(b"\r".to_vec(), b"\r".to_vec());
        Self {
            encoder: codec.clone(),
            decoder: codec,
        }
    }
}

impl<T> Encoder<T> for Codec
where
    T: AsRef<str>,
{
    type Error = <AnyDelimiterCodec as Encoder<T>>::Error;

    fn encode(
        &mut self,
        item: T,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        self.encoder.encode(item, dst)
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
