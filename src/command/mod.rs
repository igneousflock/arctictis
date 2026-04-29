pub mod bc125at;
mod no_params;
mod ok_response;

use bytes::{Bytes, BytesMut};

pub(crate) use no_params::NoParams;
pub(crate) use ok_response::OkResponse;

pub trait Command<'p> {
    const TEXT: &'static [u8];
    type Params: ParamSet<'p>;
    type Response: Response;

    fn param_set(&self) -> Self::Params;
}

pub trait ParamSet<'p>: IntoIterator<Item = &'p dyn Param> {
    fn count(&self) -> usize;
    fn size(&self) -> usize;
}

pub trait Param {
    fn write_bytes(&self, dst: &mut BytesMut);
}

pub trait Response: Sized {
    type Error: std::error::Error;

    fn deserialize(raw_values: &[Bytes]) -> Result<Self, Self::Error>;

    fn expected_field_count() -> usize;
}
