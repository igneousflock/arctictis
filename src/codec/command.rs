pub trait Command {
    fn as_bytes(&self) -> &'static [u8];
    fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
        []
    }
}

pub trait Param {
    fn as_bytes(&self) -> &[u8];
}

pub struct GetFirmwareVersion;
impl Command for GetFirmwareVersion {
    fn as_bytes(&self) -> &'static [u8] {
        b"VER"
    }
}

pub struct GetBacklight;
impl Command for GetBacklight {
    fn as_bytes(&self) -> &'static [u8] {
        b"BLT"
    }
}

pub struct SetBacklight(Backlight);

impl Command for SetBacklight {
    fn as_bytes(&self) -> &'static [u8] {
        b"BLT"
    }

    fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
        [&self.0 as &dyn Param]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Backlight {
    AlwaysOn,
    AlwaysOff,
    Keypress,
    Squelch,
    KeySquelch,
}

impl Param for Backlight {
    fn as_bytes(&self) -> &[u8] {
        match self {
            Self::AlwaysOn => b"AO",
            Self::AlwaysOff => b"AF",
            Self::Keypress => b"KY",
            Self::Squelch => b"SQ",
            Self::KeySquelch => b"KS",
        }
    }
}
