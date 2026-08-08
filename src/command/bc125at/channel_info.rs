#![expect(unused)]

use tokio_util::bytes::Bytes;

use crate::command::IntoParam;

get_set_command!(
    text: b"CIN",
    get: GetChannelInfo(ChannelIndex),
    set: SetChannelInfo,
    type: ChannelInfo(ChannelInfoError) (
        index: range ChannelIndex(1..=500 => usize),
        name: str Name(16),
        frequency: str Frequency(16),
        modulaton: enum Modulation {
            Auto => b"AUTO",
            Am => b"AM",
            Fm => b"FM",
            Nfm => b"NFM",
        },
        ctcss_dcs_status: range CtcssDcsStatus (0..=231 => usize),
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
        priority: enum Priority {
            Off => b"0",
            On => b"1",
        },
    ),
);
