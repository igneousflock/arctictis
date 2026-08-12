get_set_command!(
    text: b"PRI",
    get: GetPriorityMode,
    set: SetPriorityMode,
    single_field: enum PriorityMode {
        Off => b"0",
        On => b"1",
        PlusOn => b"2",
        DoNotDisturb => b"3",
    } PriorityModeError,
);
