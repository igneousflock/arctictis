mod private {
    pub trait Sealed {}
}

pub trait Command:
    private::Sealed + Send + Sync + Clone + std::fmt::Display + std::fmt::Debug
{
    fn as_bytes(&self) -> &'static [u8];

    fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
        []
    }
}

pub trait Param: private::Sealed + Send + Sync + std::fmt::Debug {
    fn write_bytes(&self, dst: &mut tokio_util::bytes::BytesMut);
}

impl private::Sealed for u8 {}
impl Param for u8 {
    fn write_bytes(&self, dst: &mut tokio_util::bytes::BytesMut) {
        let mut buff = itoa::Buffer::new();
        dst.extend_from_slice(buff.format(*self).as_bytes());
    }
}

macro_rules! command {
    ($cmd:literal, $name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name;

        command!(@traits $cmd, $name);

        impl Command for $name {
            fn as_bytes(&self) -> &'static [u8] { $cmd }
        }
    };
    ($cmd:literal, $name: ident ( $param_ty:ty )) => {
        #[derive(Clone, Debug)]
        pub struct $name(pub $param_ty);

        command!(@traits $cmd, $name);

        impl Command for $name {
            fn as_bytes(&self) -> &'static [u8] { $cmd }
            fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
                [&self.0 as &dyn Param]
            }
        }
    };
    ($cmd:literal, $name: ident { $($param:ident: $param_ty:ty),+ }) => {
        #[derive(Clone, Debug)]
        pub struct $name { $(pub $param: $param_ty),+ }

        command!(@traits $cmd, $name);

        impl Command for $name {
            fn as_bytes(&self) -> &'static [u8] { $cmd }
            fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
                [$(&self.$param as &dyn Param),+]
            }
        }
    };
    (@traits $cmd:literal, $name:ident) => {
        impl private::Sealed for $name {}
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                String::from_utf8_lossy($cmd).fmt(f)
            }
        }
    };
}

macro_rules! param {
    (pub enum $name:ident { $($variant:ident => $val:literal),+ $(,)? }) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            $($variant),+
        }

        param!(@traits $name);

        impl Param for $name {
            fn write_bytes(&self, dst: &mut tokio_util::bytes::BytesMut) {
                let s = match self {
                    $(Self::$variant => $val.as_slice()),+
                };
                dst.extend_from_slice(s);
            }
        }
    };
    (pub range $name:ident ($range:expr)) => {
        #[derive(Clone, Debug)]
        pub struct $name(u8);

        param!(@traits $name);

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
    (@traits $name:ident) => {
        impl private::Sealed for $name {}
    };
}

command!(b"PRG", EnterProgramMode);
command!(b"EPG", ExitProgramMode);
command!(b"MDL", GetModelInfo);
command!(b"VER", GetFirmwareVersion);

command!(b"BLT", GetBacklight);
command!(b"BLT", SetBacklight(Backlight));
param!(pub enum Backlight {
    AlwaysOn => b"AO",
    AlwaysOff => b"AF",
    Keypress => b"KY",
    Squelch => b"SQ",
    KeySquelch => b"KS",
});

command!(b"BSV", GetBatteryInfo);
command!(b"BSV", SetBatteryInfo(BatteryChargeTime));
param!(pub range BatteryChargeTime(1..=16));

command!(b"CLR", ClearAllMemory);

command!(b"BPL", GetBandPlan);
command!(b"BPL", SetBandPlan(BandPlan));
param!(pub enum BandPlan {
    Usa => b"0",
    Canada => b"1"
});

command!(b"KBP", GetKeyBeep);
command!(
    b"KBP",
    SetKeyBeep {
        beep_level: BeepLevel,
        key_lock_status: KeyLockStatus
    }
);
param!(pub enum BeepLevel {
    Auto => b"0",
    Off => b"99",
});
param!(pub enum KeyLockStatus {
    Off => b"0",
    On => b"1",
});
