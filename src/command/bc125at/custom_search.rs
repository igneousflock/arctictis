get_set_command!(
    text: b"CSG",
    get: GetCustomSearchGroups,
    set: SetCustomSearchGroups,
    single_field: str CustomSearchGroups(10) CustomSearchGroupError,
);

get_set_command!(
    text: b"CSP",
    get: GetCustomSearchSettings(SearchIndex),
    set: SetCustomSearchSettings,
    type: CustomSearchSettings(CustomSearchSettingsError) (
        search_index: range SearchIndex(1..=10 => u8),
        // TODO: custom param types
        lower_limit: range LowerLimitFrequency(250000..=5120000 => u32),
        upper_limit: range UpperLimitFrequency(250000..=5120000 => u32),
    ),
);
