#![expect(dead_code)]

use std::str::Utf8Error;

use bytes::{BufMut, BytesMut};

pub trait Command<'p> {
    const TEXT: &'static [u8];
    // type Params: IntoIterator<Item = &'a dyn Param>;
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

struct NoParams;

impl IntoIterator for NoParams {
    type Item = &'static dyn Param;
    type IntoIter = std::iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl ParamSet<'static> for NoParams {
    fn count(&self) -> usize {
        0
    }

    fn size(&self) -> usize {
        0
    }
}

pub trait Response: Sized {
    type Error: std::error::Error;

    fn parse_from_values<'f>(
        raw_values: impl Iterator<Item = &'f [u8]>,
    ) -> Result<Self, Self::Error>;
}

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

struct EnterProgramMode;

impl Command<'static> for EnterProgramMode {
    const TEXT: &'static [u8] = b"EPG";
    type Params = NoParams;
    type Response = OkResponse;

    fn param_set(&self) -> Self::Params {
        NoParams
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FirmwareVersionError {
    #[error("invalid UTF-8 bytes")]
    Utf8Error(#[from] Utf8Error),
    #[error("expected one response field")]
    WrongNumberOfFields,
}

struct FirmwareVersion(String);

impl Response for FirmwareVersion {
    type Error = FirmwareVersionError;
    fn parse_from_values<'f>(
        mut raw_values: impl Iterator<Item = &'f [u8]>,
    ) -> Result<Self, Self::Error> {
        let bytes = raw_values
            .next()
            .ok_or(FirmwareVersionError::WrongNumberOfFields)?;

        let version = str::from_utf8(bytes)?;

        if raw_values.next().is_some() {
            return Err(FirmwareVersionError::WrongNumberOfFields);
        }

        Ok(Self(version.to_string()))
    }
}

struct GetFirmwareVersion;

impl Command<'static> for GetFirmwareVersion {
    const TEXT: &'static [u8] = b"VER";
    type Params = NoParams;
    type Response = FirmwareVersion;

    fn param_set(&self) -> Self::Params {
        NoParams
    }
}

fn send_command<'a, Cmd: Command<'a>>(dst: &mut BytesMut, item: &Cmd) {
    let params = item.param_set();
    let est_len = Cmd::TEXT.len() + params.count() + params.size() + 1;
    dst.reserve(est_len);

    dst.extend_from_slice(Cmd::TEXT);

    for param in item.param_set() {
        dst.put_u8(b',');
        param.write_bytes(dst);
    }

    dst.put_u8(b'\r');
}

#[derive(Debug, thiserror::Error)]
enum ResponseError<E> {
    #[error("malformed response")]
    Malformed,
    #[error("response is for wrong command")]
    WrongCommand,
    #[error(transparent)]
    InvalidFields(#[from] E),
}

fn parse_response<'a, 'b, Cmd: Command<'a>>(
    response: &'b [u8],
) -> Result<Cmd::Response, ResponseError<<Cmd::Response as Response>::Error>> {
    let mut fields = response.split(|b| *b == b',');

    let Some(cmd) = fields.next() else {
        return Err(ResponseError::Malformed);
    };

    if cmd != Cmd::TEXT {
        return Err(ResponseError::WrongCommand);
    }

    let response = Cmd::Response::parse_from_values(fields)?;

    Ok(response)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sending_works() {
        let mut buf = BytesMut::new();
        let cmd = GetFirmwareVersion;
        send_command(&mut buf, &cmd);

        let buf = buf.freeze();
        assert_eq!(buf.as_ref(), b"VER\r");
    }

    #[test]
    fn receiving_works() {
        let response = b"VER,FOOBAR";

        #[allow(clippy::unwrap_used)]
        parse_response::<GetFirmwareVersion>(response).unwrap();
    }
}
