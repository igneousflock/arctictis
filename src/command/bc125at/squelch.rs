use std::{num::ParseIntError, str::Utf8Error};

use crate::command::{OkResponse, command, range_param, range_response};

#[derive(Debug, thiserror::Error)]
pub enum SquelchLevelError {
    #[error("invalid UTF-8 bytes")]
    Utf8Error(#[from] Utf8Error),
    #[error(transparent)]
    Parse(#[from] ParseIntError),
    #[error("squelch must be between [0..15], got `{0}`")]
    InvalidSquelch(u8),
}

range_param!(SquelchLevel(0..=15): u8);
range_response!(SquelchLevel => SquelchLevelError : InvalidSquelch);

command!(b"SQL": GetSquelchLevel => SquelchLevel);
command!(b"SQL": SetSquelchLevel(SquelchLevel) => OkResponse);
