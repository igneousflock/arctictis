// TODO: move Frequency to shared module
use crate::{Command, OkResponse, bc125at::channel_info::Frequency, command::SingleParam};

pub struct LockoutFrequency(pub Frequency);

impl Command for LockoutFrequency {
    const TEXT: &'static [u8] = b"LOF";

    type Params = SingleParam<Frequency>;

    type Response = OkResponse;

    fn params(self) -> Self::Params {
        SingleParam(self.0)
    }
}
