pub mod bc125at;
#[macro_use]
mod macros;
mod no_params;
mod ok_response;

use tokio_util::bytes::{BufMut, Bytes, BytesMut};

pub use ok_response::{OkResponse, OkResponseError};

pub(crate) use macros::{command, range_param, range_response};
pub(crate) use no_params::NoParams;

use crate::codec::PARAM_DELIMITER;

/// Defines a command that can be sent to a scanner, including params and the response type.
pub trait Command {
    /// The three-character text of the command itself (for example: `VOL`, `EPG`)
    const TEXT: &'static [u8];

    /// The type of the parameters sent with this command
    type Params: Params;
    /// The values this command expects from the scanner in response
    type Response: Response;

    fn params(&self) -> &Self::Params;
}

/// Tells the codec how to parse a response to a command
pub trait Response: Sized {
    /// The type returned when parsing fails
    type Error: std::error::Error;

    /// Given a list of raw values, parses the response into a concrete type.
    ///
    /// The length of `raw_values` will always be equal to [`Response::expected_field_count`]
    fn deserialize(raw_values: &[Bytes]) -> Result<Self, Self::Error>;

    /// The number of values expected by this command, not including the command itself.
    fn expected_field_count() -> usize;
}

pub trait Params {
    /// How many parameters are included in this command
    ///
    /// This is used to determine how much space to reserve for `,` delimiters in the command
    /// buffer before serialization.
    fn count(&self) -> usize;

    /// The maximum number of bytes these parameters could take up, not including their `,`
    /// delimiters
    ///
    /// * For numeric values, this is usually the maximum number of digits in their decimal representation
    /// * For string values, this is their max length
    ///
    /// This is used to determine how much space to reserve for the parameters in the command buffer
    /// before serialization.
    fn max_size(&self) -> usize;

    /// Serialize the parameters into a given buffer
    ///
    /// Implementors should sequentially call [`ParamBuffer::serialize_param`] with each parameter. The
    /// buffer will handle any delimiters.
    fn serialize_to(&self, buffer: ParamBuffer);
}

/// Thin wrapper around [`BytesMut`] for parameter serialization
pub struct ParamBuffer<'a>(&'a mut BytesMut);

impl<'a> ParamBuffer<'a> {
    pub fn new(bytes: &'a mut BytesMut) -> Self {
        Self(bytes)
    }

    /// Add a `,` delimiter before pushing the given bytes to the buffer
    pub fn serialize_param(&mut self, bytes: &[u8]) {
        self.0.put_u8(PARAM_DELIMITER);
        self.0.extend_from_slice(bytes);
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    /// Shorthand for commands to deserialize a response from raw bytes
    pub fn deserialize<R: Response, T: AsRef<[u8]>>(val: &'static T) -> Result<R, R::Error> {
        R::deserialize(&[Bytes::from(val.as_ref())])
    }
}
