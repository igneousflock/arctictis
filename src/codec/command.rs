#[macro_use]
mod macros;

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

gen_command!(b"PRG", EnterProgramMode);
gen_command!(b"EPG", ExitProgramMode);
gen_command!(b"MDL", GetModelInfo);
gen_command!(b"VER", GetFirmwareVersion);

gen_command!(b"BLT", GetBacklight);
gen_command!(b"BLT", SetBacklight(Backlight));
gen_param!(pub enum Backlight {
    AlwaysOn => b"AO",
    AlwaysOff => b"AF",
    Keypress => b"KY",
    Squelch => b"SQ",
    KeySquelch => b"KS",
});

gen_command!(b"BSV", GetBatteryInfo);
gen_command!(b"BSV", SetBatteryInfo(BatteryChargeTime));
gen_param!(pub range BatteryChargeTime(1..=16));

gen_command!(b"CLR", ClearAllMemory);

gen_command!(b"BPL", GetBandPlan);
gen_command!(b"BPL", SetBandPlan(BandPlan));
gen_param!(pub enum BandPlan {
    Usa => b"0",
    Canada => b"1"
});

gen_command!(b"KBP", GetKeyBeep);
gen_command!(
    b"KBP",
    SetKeyBeep {
        beep_level: BeepLevel,
        key_lock_status: KeyLockStatus
    }
);
gen_param!(pub enum BeepLevel {
    Auto => b"0",
    Off => b"99",
});
gen_param!(pub enum KeyLockStatus {
    Off => b"0",
    On => b"1",
});
