use std::str::Utf8Error;

use crate::command::{Response, command};

#[derive(Debug, thiserror::Error)]
pub enum FirmwareVersionError {
    #[error("invalid UTF-8 bytes")]
    Utf8Error(#[from] Utf8Error),
}

#[derive(Debug)]
pub struct FirmwareVersion(pub String);

impl Response for FirmwareVersion {
    type Error = FirmwareVersionError;
    fn deserialize(raw_values: &[bytes::Bytes]) -> Result<Self, Self::Error> {
        let version = str::from_utf8(&raw_values[0])?.to_string();
        Ok(Self(version))
    }

    fn expected_field_count() -> usize {
        1
    }
}

command!(b"VER": GetFirmwareVersion => FirmwareVersion);
