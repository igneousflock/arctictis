get_set_command!(
    text: b"BLT",
    get: GetBacklight,
    set: SetBacklight,
    single_field: enum Backlight {
        AlwaysOn => b"AO",
        Keypress => b"KY",
        Squelch => b"SQ",
        KeyOrSquelch => b"KS",
        AlwaysOff => b"AF",
    } BacklightError,
);

get_set_command!(
    text: b"BSV",
    get: GetBatteryInfo,
    set: SetBatteryInfo,
    single_field: range BatteryChargeTime(1..=16 => u8) BatteryInfoError,
);

get_set_command!(
    text: b"CNT",
    get: GetLcdContrast,
    set: SetLcdContrast,
    single_field: range LcdContrast(1..=15 => u8) LcdContrastError,
);

get_set_command! {
    text: b"KBP",
    get: GetKeyBeepSetting,
    set: SetKeyBeep,
    type: KeyBeepSetting(KeyBeepSettingError) (
        beep_level: enum BeepLevel {
            Auto => b"0",
            Off => b"99",
        },
        key_lock_status: enum KeyLockStatus {
            Off => b"0",
            On => b"1",
        },
    ),
    non_program_mode: true,
}

get_set_command!(
    text: b"SQL",
    get: GetSquelch,
    set: SetSquelch,
    single_field: range Squelch(0..=15 => u8) SquelchError,
    non_program_mode: true,
);

get_set_command!(
    text: b"VOL",
    get: GetVolume,
    set: SetVolume,
    single_field: range Volume(0..=15 => u8) VolumeError,
    non_program_mode: true,
);

get_set_command!(
    text: b"WXS",
    get: GetWeatherSettings,
    set: SetWeatherSettings,
    single_field: enum WeatherAlertPriority {
        Off => b"0",
        On => b"1",
    } WeatherSettingsError,
);
