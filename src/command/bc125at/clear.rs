use crate::{Command, OkResponse, command::NoParams};

#[derive(Debug, Clone, Copy)]
pub struct ClearAllMemory;

impl Command for ClearAllMemory {
    const TEXT: &'static [u8] = b"CLR";

    type Params = NoParams;

    type Response = OkResponse;

    fn params(self) -> Self::Params {
        NoParams
    }
}
