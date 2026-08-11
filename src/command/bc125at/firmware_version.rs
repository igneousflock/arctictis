use std::string::FromUtf8Error;

use crate::{
    Command,
    command::{NoParams, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum FirmwareVersionError {
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error("expected one field")]
    Malformed,
}

#[derive(Debug)]
pub struct FirmwareVersion(pub String);

impl Response for FirmwareVersion {
    type Error = FirmwareVersionError;

    fn deserialize<'i, I: Iterator<Item = &'i tokio_util::bytes::Bytes>>(
        mut raw_values: I,
    ) -> Result<Self, Self::Error> {
        if let Some(val) = raw_values.next()
            && raw_values.next().is_none()
        {
            let utf8 = String::from_utf8(val.to_vec())?;
            Ok(Self(utf8))
        } else {
            Err(FirmwareVersionError::Malformed)
        }
    }

    fn expected_field_count() -> usize {
        1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetFirmwareVersion;

impl Command for GetFirmwareVersion {
    const TEXT: &'static [u8] = b"VER";

    type Params = NoParams;

    type Response = FirmwareVersion;

    fn params(self) -> Self::Params {
        NoParams
    }
}
