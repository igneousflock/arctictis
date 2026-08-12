get_set_command!(
    text: b"BPL",
    get: GetBandPlan,
    set: SetBandPlan,
    single_field: enum BandPlan {
        Usa => b"0",
        Canada => b"1",
    } BandPlanError,
);
