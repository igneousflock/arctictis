use crate::command::Response;

#[derive(Debug, thiserror::Error)]
pub enum OkResponseError {
    #[error("expected `OK`, got `{0}`")]
    UnexpectedValue(String),
    #[error("expected one response field")]
    WrongNumberOfFields,
}

pub struct OkResponse;

impl Response for OkResponse {
    type Error = OkResponseError;

    fn parse_from_values<'f>(
        mut raw_values: impl Iterator<Item = &'f [u8]>,
    ) -> Result<Self, Self::Error> {
        let val = raw_values
            .next()
            .ok_or(OkResponseError::WrongNumberOfFields)?;

        if val != b"OK" {
            return Err(OkResponseError::UnexpectedValue(
                String::from_utf8_lossy(val).to_string(),
            ));
        }

        if raw_values.next().is_some() {
            return Err(OkResponseError::WrongNumberOfFields);
        }

        Ok(Self)
    }
}
