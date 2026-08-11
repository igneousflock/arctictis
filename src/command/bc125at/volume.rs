get_set_command!(
    text: b"VOL",
    get: GetVolume,
    set: SetVolume,
    single_field: range Volume(0..=15 => u8) VolumeError,
    non_program_mode: true,
);
