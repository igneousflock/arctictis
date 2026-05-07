use crate::command::{OkResponse, command, range_param, range_response};

range_param!(LcdContrast(1..=15): u8);
range_response!(LcdContrast => LcdContrastError : InvalidLcdContrast("LCD contrast must be between [1..15], got `{0}`"));

command!(b"CNT": GetLcdContrast => LcdContrast);
command!(b"CNT": SetLcdContrast(LcdContrast) => OkResponse);
