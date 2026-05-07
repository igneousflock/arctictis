use tokio_util::bytes::Bytes;

use crate::command::Response;

#[derive(Debug, thiserror::Error)]
#[error("expected `OK`")]
pub struct OkResponseError;

#[derive(Clone, Copy, Debug)]
pub struct OkResponse;

impl Response for OkResponse {
    type Error = OkResponseError;

    fn deserialize(raw_values: &[Bytes]) -> Result<Self, Self::Error> {
        if raw_values[0] != b"OK".as_ref() {
            return Err(OkResponseError);
        }

        Ok(Self)
    }

    fn expected_field_count() -> usize {
        1
    }
}
