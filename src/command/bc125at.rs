use std::str::Utf8Error;

use crate::command::{Command, NoParams, OkResponse, Response};

#[derive(Debug)]
pub struct EnterProgramMode;

impl Command<'static> for EnterProgramMode {
    const TEXT: &'static [u8] = b"EPG";
    type Params = NoParams;
    type Response = OkResponse;

    fn param_set(&self) -> Self::Params {
        NoParams
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FirmwareVersionError {
    #[error("invalid UTF-8 bytes")]
    Utf8Error(#[from] Utf8Error),
}

#[derive(Debug)]
pub struct FirmwareVersion(pub String);

impl Response for FirmwareVersion {
    type Error = FirmwareVersionError;
    fn parse_from_values(raw_values: &[bytes::Bytes]) -> Result<Self, Self::Error> {
        let version = str::from_utf8(&raw_values[0])?.to_string();
        Ok(Self(version))
    }

    fn expected_field_count() -> usize {
        1
    }
}

pub struct GetFirmwareVersion;

impl Command<'static> for GetFirmwareVersion {
    const TEXT: &'static [u8] = b"VER";
    type Params = NoParams;
    type Response = FirmwareVersion;

    fn param_set(&self) -> Self::Params {
        NoParams
    }
}
