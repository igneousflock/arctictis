use std::string::FromUtf8Error;

use crate::{
    Command, NonProgramModeCommand,
    command::{NoParams, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error("expected one field")]
    Malformed,
}

#[derive(Debug)]
pub struct Model(pub String);

impl Response for Model {
    type Error = ModelError;

    fn deserialize<'i, I: Iterator<Item = &'i tokio_util::bytes::Bytes>>(
        mut raw_values: I,
    ) -> Result<Self, Self::Error> {
        if let Some(val) = raw_values.next()
            && raw_values.next().is_none()
        {
            let utf8 = String::from_utf8(val.to_vec())?;
            Ok(Self(utf8))
        } else {
            Err(ModelError::Malformed)
        }
    }

    fn expected_field_count() -> usize {
        1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetModel;

impl Command for GetModel {
    const TEXT: &'static [u8] = b"VER";

    type Params = NoParams;

    type Response = Model;

    fn params(self) -> Self::Params {
        NoParams
    }
}
impl NonProgramModeCommand for GetModel {}
