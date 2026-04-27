use std::fmt::Write;

pub trait Command {
    fn as_bytes(&self) -> &'static [u8];

    fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
        []
    }
}

pub trait Param {
    fn write_bytes(&self, dst: &mut tokio_util::bytes::BytesMut);
}

impl Param for u8 {
    fn write_bytes(&self, dst: &mut tokio_util::bytes::BytesMut) {
        write!(dst, "{self}").unwrap()
    }
}

// TODO: don't require named params
macro_rules! command {
    ($cmd:literal, $name:ident) => {
        pub struct $name;
        impl Command for $name {
            fn as_bytes(&self) -> &'static [u8] {
                $cmd
            }
        }
    };
    ($cmd:literal, $name: ident { $($param:ident: $param_ty:ty),+ }) => {
        pub struct $name { $(pub $param: $param_ty),+ }
        impl Command for $name {
            fn as_bytes(&self) -> &'static [u8] {
                $cmd
            }
            fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
                [$(&self.$param as &dyn Param),+]
            }
        }
    };
}

macro_rules! param {
    (pub enum $name:ident { $($variant:ident => $val:literal),+ }) => {
        pub enum $name {
            $($variant),+
        }
        impl Param for $name {
            fn write_bytes(&self, dst: &mut tokio_util::bytes::BytesMut) {
                let s = match self {
                    $(Self::$variant => $val),+
                };
                dst.extend_from_slice(s);
            }
        }
    };
    (pub range $name:ident ($range:expr)) => {
        pub struct $name(u8);
        impl $name {
            pub fn new(value: u8) -> Self {
                assert!($range.contains(&value));
                Self(value)
            }
        }
        impl Param for $name {
            fn write_bytes(&self, dst: &mut tokio_util::bytes::BytesMut) {
                self.0.write_bytes(dst);
            }
        }
    };
}

command!(b"PRG", EnterProgramMode);
command!(b"EPG", ExitProgramMode);
command!(b"MDL", GetModelInfo);
command!(b"VER", GetFirmwareVersion);

command!(b"BLT", GetBacklight);
command!(
    b"BLT",
    SetBacklight {
        backlight: Backlight
    }
);

param! {
    pub enum Backlight {
        AlwaysOn => b"AO",
        AlwaysOff => b"AF",
        Keypress => b"KY",
        Squelch => b"SQ",
        KeySquelch => b"KS"
    }
}

command!(b"BSV", GetBatteryInfo);
command!(
    b"BSV",
    SetBatteryInfo {
        charge_time: BatteryChargeTime
    }
);
param!(pub range BatteryChargeTime(1..=16));
