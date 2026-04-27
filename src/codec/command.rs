pub trait Command {
    fn as_bytes(&self) -> &'static [u8];

    fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
        []
    }
}

pub trait Param {
    fn as_bytes(&self) -> &[u8];
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
        pub struct $name { $($param: $param_ty),+ }
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
            fn as_bytes(&self) -> &[u8] {
                match self {
                    $(Self::$variant => $val),+
                }
            }
        }
    }
}

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
