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

#[derive(Clone, Copy, Debug, strum::IntoStaticStr, strum::EnumString)]
pub enum BandPlan {
    #[strum(to_string = "0")]
    Usa,
    #[strum(to_string = "1")]
    Canada,
}

#[derive(Clone, Copy, Debug)]
pub struct KeyBeepSettings {
    pub beep_level: KeyBeepLevel,
    pub lock_status: bool,
}

#[derive(Clone, Copy, Debug, strum::IntoStaticStr, strum::EnumString)]
pub enum KeyBeepLevel {
    #[strum(to_string = "0")]
    Auto,
    #[strum(to_string = "99")]
    Off,
}

#[derive(Clone, Copy, Debug, strum::IntoStaticStr, strum::EnumString)]
pub enum PriorityMode {
    #[strum(to_string = "0")]
    Off,
    #[strum(to_string = "1")]
    On,
    #[strum(to_string = "2")]
    PlusOn,
    #[strum(to_string = "3")]
    Dnd,
}
