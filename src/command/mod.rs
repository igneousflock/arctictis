pub mod bc125at;
mod no_params;
mod ok_response;

use tokio_util::bytes::Bytes;

pub use no_params::NoParams;
pub use ok_response::{OkResponse, OkResponseError};

pub trait Command {
    const TEXT: &'static [u8];

    type Params: Params;
    type Response: Response;

    fn params(self) -> Self::Params;
}

pub trait Params: IntoIterator<Item = &'static [u8]> {
    fn size_hint(&self) -> usize;
}

pub trait Response: Sized {
    type Error: std::error::Error;

    fn deserialize<'i, I: Iterator<Item = &'i Bytes>>(raw_values: I) -> Result<Self, Self::Error>;

    fn expected_field_count() -> usize;
}
