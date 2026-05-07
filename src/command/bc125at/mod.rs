use std::{num::ParseIntError, str::Utf8Error};

use crate::command::{OkResponse, Params, Response, command};

use super::ParamBuffer;

command!(b"PRG": EnterProgramMode);
command!(b"EPG": ExitProgramMode);

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

#[derive(Debug, thiserror::Error)]
pub enum VolumeLevelError {
    #[error("invalid UTF-8 bytes")]
    Utf8Error(#[from] Utf8Error),
    #[error(transparent)]
    Parse(#[from] ParseIntError),
    #[error("volume must be between [0..15], got `{0}`")]
    InvalidVolume(u8),
}

#[derive(Debug)]
pub struct VolumeLevel(u8);

impl VolumeLevel {
    pub fn new(level: u8) -> Option<Self> {
        (0..=15).contains(&level).then_some(Self(level))
    }
}

impl Params for VolumeLevel {
    fn count(&self) -> usize {
        1
    }

    fn total_size(&self) -> usize {
        if self.0 >= 10 { 2 } else { 1 }
    }

    fn serialize_to(&self, mut buffer: ParamBuffer) {
        let mut serialized = itoa::Buffer::new();
        buffer.serialize_param(serialized.format(self.0).as_bytes());
    }
}

impl Response for VolumeLevel {
    type Error = VolumeLevelError;

    fn deserialize(raw_values: &[bytes::Bytes]) -> Result<Self, Self::Error> {
        let level = str::from_utf8(&raw_values[0])?.parse()?;
        Self::new(level).ok_or(VolumeLevelError::InvalidVolume(level))
    }

    fn expected_field_count() -> usize {
        1
    }
}

command!(b"VOL": GetVolumeLevel => VolumeLevel);
command!(b"VOL": SetVolumeLevel(VolumeLevel) => OkResponse);
