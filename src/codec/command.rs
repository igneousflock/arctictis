use crate::{
    KeyBeepSettings, PriorityMode,
    codec::misc::{Backlight, BandPlan},
};

#[derive(Clone, Copy, Debug, strum::IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Command {
    // Program control mode
    /// Enter program mode
    Prg,
    /// Exit program mode
    Epg,

    // System information
    /// Get model info
    Mdl,
    /// Get firmware version
    Ver,

    // System settings
    /// Get/set backlight trigger event
    Blt(Option<Backlight>),
    /// Get/set battery charge time
    Bsv(Option<u8>),
    /// Clear all memory
    ///
    /// Takes about 20 seconds to execute
    Clr,
    /// Get/set band plan
    ///
    /// Affects frequency step. Issue before frequency programming.
    Bpl(Option<BandPlan>),
    /// Get/set key beep
    Kbp(Option<KeyBeepSettings>), // TODO: optional param
    /// Get/set priority mode
    Pri(Option<PriorityMode>), // TODO: optional param

    // Scan Settings
    // Scg,
    // Dch,
    // Cin,

    // Search/close call settings
    // Sco
    // Glf
    // Ulf
    // Lof
    // Clc

    // Service/custom search settings
    // Ssg
    // Csg
    // Csp

    // Wxs
    // Cnt
    // Vol
    // Sql

    // undocumented?
    /// Power OFF
    Pof,
    /// Get status
    Sts,
}
