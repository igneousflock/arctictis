use std::{num::ParseIntError, str::Utf8Error};

use crate::command::{OkResponse, command, range_param, range_response};

#[derive(Debug, thiserror::Error)]
pub enum VolumeLevelError {
    #[error("invalid UTF-8 bytes")]
    Utf8Error(#[from] Utf8Error),
    #[error(transparent)]
    Parse(#[from] ParseIntError),
    #[error("volume must be between [0..15], got `{0}`")]
    InvalidVolume(u8),
}

range_param!(VolumeLevel(0..=15): u8);
range_response!(VolumeLevel => VolumeLevelError : InvalidVolume);

command!(b"VOL": GetVolumeLevel => VolumeLevel);
command!(b"VOL": SetVolumeLevel(VolumeLevel) => OkResponse);
