#[derive(Clone, Copy, Debug, strum::IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Command {
    /// Get current talkgroup ID status
    Gid,
    /// Power OFF
    Pof,
    /// Get status
    Sts,
    /// Get model info
    Mdl,
    /// Get firmware version
    Ver,

    /// Enter program mode
    Prg,
    /// Exit program mode
    Epg,

    /// Get/set backlight
    Blt, // TODO: optional param
    /// Get/set battery save
    Bsv, // TODO: optional param
    /// Clear all memory
    ///
    /// Takes about 20 seconds to execute
    Clr,
    /// Get/set key beep
    Kbp, // TODO: optional param
    /// Get/set opening message
    Oms, // TODO: optional param
    /// Get/set priority mode
    Pri, // TODO: optional param
    /// Get/set auto gain control
    Agv, // TODO: optional param
    /// Get/set bar antenna
    Bar, // TODO: optional param

    /// Get system count
    Sct,
    /// Get system index head
    Sih,
    /// Get system index tail
    Sit,

    /// Get/set quick lockout
    Qsl, // TODO: optional params
    Vol,
}
