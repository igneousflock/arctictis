#![expect(unused)]

use tokio_util::bytes::Bytes;

use crate::{
    Command, OkResponse,
    command::{IntoParam, SingleParam},
};

get_set_command!(
    text: b"CIN",
    get: GetChannelInfo(ChannelIndex),
    set: SetChannelInfo,
    type: ChannelInfo(ChannelInfoError) (
        index: range ChannelIndex(1..=500 => usize),
        name: str Name(16),
        // TODO: frequency validation
        frequency: str Frequency(8),
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

// TODO: move ChannelIndex to a shared module
pub struct DeleteChannel(pub ChannelIndex);

impl Command for DeleteChannel {
    const TEXT: &'static [u8] = b"DCH";

    type Params = SingleParam<ChannelIndex>;

    type Response = OkResponse;

    fn params(self) -> Self::Params {
        SingleParam(self.0)
    }
}
