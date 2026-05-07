use crate::command::{OkResponse, command, range_param, range_response};

range_param!(BatteryChargeTime(1..=16): u8);
range_response!(BatteryChargeTime => BatteryChargeTimeError : InvalidBatteryChargeTime("volume must be between [1..16], got `{0}`"));

command!(b"BSV": GetBatteryInfo => BatteryChargeTime);
command!(b"BSV": SetBatteryInfo(BatteryChargeTime) => OkResponse);
