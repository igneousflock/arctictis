get_set_command! {
    text: b"SCO",
    get: GetSearchCloseCallSettings,
    set: SetSearchCloseCallSettings,
    type: SearchCloseCallSettings(SearchCloseCallError) (
        // TODO: move these to a shared module
        delay: enum Delay {
            Neg10 => b"-10",
            Neg5 => b"-5",
            Zero => b"0",
            One => b"1",
            Two => b"2",
            Three => b"3",
            Four => b"4",
            Five => b"5",
        },
        lockout: enum Lockout {
            Unlocked => b"0",
            Lockout => b"1",
        },
    ),
    non_program_mode: true,
}

get_set_command!(
    text: b"CLC",
    get: GetCloseCallSettings,
    set: SetCloseCallSettings,
    type: CloseCallSettings(CloseCallSettingsError) (
        cc_mode: enum CcMode {
            Off => b"0",
            Priority => b"1",
            DoNotDisturb => b"2",
            CloseCallOnly => b"3",
        },
        alert_beep: enum AlertBeep {
            Off => b"0",
            On => b"1",
        },
        alert_light: enum AlertLight {
            Off => b"0",
            On => b"1",
        },
        // TODO: close call band field validation
        cc_band: str CloseCallBand(5),
        lockout_cc_hits_with_scan: enum LockoutCcHitsWithScan {
            Lockout => b"0",
            Unlocked => b"1",
        }
    ),
);
