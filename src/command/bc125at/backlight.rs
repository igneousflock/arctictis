use crate::{
    OkResponse,
    command::{command, macros::enum_param_response},
};

enum_param_response!(Backlight {
    AlwaysOn => b"AO",
    AlwaysOff => b"AF",
    Keypress => b"KY",
    Squelch => b"SQ",
    KeypressOrSquelch => b"KS",
}: BacklightError("invalid backlight setting"));

command!(b"BLT": GetBacklight => Backlight);
command!(b"BLT": SetBacklight(Backlight) => OkResponse);
