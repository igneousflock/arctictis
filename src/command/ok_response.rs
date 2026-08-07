use tokio_util::bytes::Bytes;

use crate::command::Response;

#[derive(Debug, Clone, Copy)]
pub struct OkResponse;
#[derive(Debug, thiserror::Error)]
#[error("expected `OK`")]
pub struct OkResponseError;

impl Response for OkResponse {
    type Error = OkResponseError;

    fn deserialize<'i, I: Iterator<Item = &'i Bytes>>(
        mut raw_values: I,
    ) -> Result<Self, Self::Error> {
        if let Some(val) = raw_values.next()
            && val.as_ref() == b"OK"
            && raw_values.next().is_none()
        {
            Ok(Self)
        } else {
            Err(OkResponseError)
        }
    }

    fn expected_field_count() -> usize {
        1
    }
}
