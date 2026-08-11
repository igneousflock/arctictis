use crate::{Command, OkResponse, command::NoParams};

#[derive(Clone, Copy, Debug)]
pub struct EnterProgramMode;

impl Command for EnterProgramMode {
    const TEXT: &'static [u8] = b"PRG";

    type Params = NoParams;

    type Response = OkResponse;

    fn params(self) -> Self::Params {
        NoParams
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ExitProgramMode;

impl Command for ExitProgramMode {
    const TEXT: &'static [u8] = b"EPG";

    type Params = NoParams;

    type Response = OkResponse;

    fn params(self) -> Self::Params {
        NoParams
    }
}
