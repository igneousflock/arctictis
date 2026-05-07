use crate::command::{OkResponse, command, range_param, range_response};

range_param!(SquelchLevel(0..=15): u8);
range_response!(SquelchLevel => SquelchLevelError : InvalidSquelch("squelch must be between [0..15], got `{0}`"));

command!(b"SQL": GetSquelchLevel => SquelchLevel);
command!(b"SQL": SetSquelchLevel(SquelchLevel) => OkResponse);
