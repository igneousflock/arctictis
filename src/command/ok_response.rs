use crate::command::Response;

#[derive(Debug, thiserror::Error)]
pub enum OkResponseError {
    #[error("expected `OK`")]
    UnexpectedValue,
    #[error("expected one response field")]
    WrongNumberOfFields,
}

#[derive(Clone, Copy, Debug)]
pub struct OkResponse;

impl Response for OkResponse {
    type Error = OkResponseError;

    fn deserialize(raw_values: &[bytes::Bytes]) -> Result<Self, Self::Error> {
        if raw_values[0] != b"OK".as_ref() {
            return Err(OkResponseError::UnexpectedValue);
        }

        Ok(Self)
    }

    fn expected_field_count() -> usize {
        1
    }
}
