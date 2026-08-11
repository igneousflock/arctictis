pub mod bc125at;
mod no_params;
mod ok_response;
mod single_param;

use tokio_util::bytes::Bytes;

pub use no_params::NoParams;
pub use ok_response::{OkResponse, OkResponseError};
pub use single_param::SingleParam;

pub trait Command {
    const TEXT: &'static [u8];

    type Params: Params;
    type Response: Response;

    fn params(self) -> Self::Params;
}

pub trait Params: IntoIterator<Item = Bytes> {
    fn size_hint(&self) -> usize;
}

pub trait IntoParam {
    fn into_param(self) -> Bytes;
    fn size_hint(&self) -> usize;
}

pub trait ResponseField: Sized {
    fn deserialize(raw: &[u8]) -> Option<Self>;
}

pub trait Response: Sized {
    type Error: std::error::Error;

    fn deserialize<'i, I: Iterator<Item = &'i Bytes>>(raw_values: I) -> Result<Self, Self::Error>;

    fn expected_field_count() -> usize;
}

pub type CommandResponseError<Cmd> = <<Cmd as Command>::Response as Response>::Error;

pub trait NonProgramModeCommand: Command {}

mod npmc {
    use crate::bc125at::{
        firmware_version::GetFirmwareVersion,
        key_beep::{GetKeyBeepSetting, SetKeyBeep},
        volume::{GetVolume, SetVolume},
    };

    use super::NonProgramModeCommand;

    impl NonProgramModeCommand for GetFirmwareVersion {}
    impl NonProgramModeCommand for GetKeyBeepSetting {}
    impl NonProgramModeCommand for SetKeyBeep {}
    impl NonProgramModeCommand for GetVolume {}
    impl NonProgramModeCommand for SetVolume {}
}
