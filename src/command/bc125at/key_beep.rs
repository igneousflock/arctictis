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
}
