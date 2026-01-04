#[derive(Clone, Copy, Debug, strum::IntoStaticStr, strum::EnumString)]
pub enum Backlight {
    #[strum(to_string = "IF")]
    Infinite,
    #[strum(to_string = "10")]
    Sec10,
    #[strum(to_string = "30")]
    Sec30,
    #[strum(to_string = "KY")]
    Keypress,
    #[strum(to_string = "SQ")]
    Squelch,
}
