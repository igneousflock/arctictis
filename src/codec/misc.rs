#[derive(Clone, Copy, Debug, strum::IntoStaticStr, strum::EnumString)]
pub enum Backlight {
    #[strum(to_string = "AO")]
    AlwaysOn,
    #[strum(to_string = "AF")]
    AlwaysOff,
    #[strum(to_string = "KY")]
    Keypress,
    #[strum(to_string = "SQ")]
    Squelch,
    #[strum(to_string = "KS")]
    KeySquelch,
}
