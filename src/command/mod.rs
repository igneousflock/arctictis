pub mod bc125at;
#[macro_use]
mod macros;
mod no_params;
mod ok_response;

use tokio_util::bytes::{BufMut, Bytes, BytesMut};

pub(crate) use macros::{command, range_param, range_response};
pub(crate) use no_params::NoParams;
pub use ok_response::{OkResponse, OkResponseError};

use crate::codec::PARAM_DELIMITER;

pub trait Command {
    const TEXT: &'static [u8];
    type Params: Params;
    type Response: Response;

    fn params(&self) -> &Self::Params;
}

pub trait Response: Sized {
    type Error: std::error::Error;

    fn deserialize(raw_values: &[Bytes]) -> Result<Self, Self::Error>;

    fn expected_field_count() -> usize;
}

pub trait Params {
    fn count(&self) -> usize;
    fn total_size(&self) -> usize;
    fn serialize_to(&self, buffer: ParamBuffer);
}

pub struct ParamBuffer<'a>(&'a mut BytesMut);

impl<'a> ParamBuffer<'a> {
    pub fn new(bytes: &'a mut BytesMut) -> Self {
        Self(bytes)
    }

    pub fn serialize_param(&mut self, bytes: &[u8]) {
        self.0.put_u8(PARAM_DELIMITER);
        self.0.extend_from_slice(bytes);
    }
}
