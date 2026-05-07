use crate::command::{OkResponse, command, range_param, range_response};

range_param!(VolumeLevel(0..=15): u8);
range_response!(VolumeLevel => VolumeLevelError : InvalidVolume("volume must be between [0..15], got `{0}`"));

command!(b"VOL": GetVolumeLevel => VolumeLevel);
command!(b"VOL": SetVolumeLevel(VolumeLevel) => OkResponse);
