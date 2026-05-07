use crate::command::{command, macros::string_response};

string_response!(FirmwareVersion => FirmwareVersionError);

command!(b"VER": GetFirmwareVersion => FirmwareVersion);
