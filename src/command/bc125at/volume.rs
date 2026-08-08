use tokio_util::bytes::Bytes;

use crate::{
    Command, OkResponse,
    command::{IntoParam, NoParams, Response, single_param::SingleParam},
};

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("invalid volume")]
pub struct VolumeError;

#[derive(Clone, Copy, Debug)]
pub struct Volume(u8);

impl Volume {
    pub fn new(volume: u8) -> Option<Self> {
        (0..=15).contains(&volume).then_some(Self(volume))
    }
}

impl IntoParam for Volume {
    fn into_param(self) -> Bytes {
        Bytes::from(format!("{}", self.0))
    }
}

impl Response for Volume {
    type Error = VolumeError;

    fn deserialize<'i, I: Iterator<Item = &'i Bytes>>(
        mut raw_values: I,
    ) -> Result<Self, Self::Error> {
        if let Some(val) = raw_values.next()
            && raw_values.next().is_none()
        {
            let level = str::from_utf8(val.as_ref())
                .map_err(|_| VolumeError)?
                .parse()
                .map_err(|_| VolumeError)?;
            Volume::new(level).ok_or(VolumeError)
        } else {
            Err(VolumeError)
        }
    }

    fn expected_field_count() -> usize {
        1
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GetVolume;

impl Command for GetVolume {
    const TEXT: &'static [u8] = b"VOL";

    type Params = NoParams;

    type Response = Volume;

    fn params(self) -> Self::Params {
        NoParams
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SetVolume(pub Volume);

impl Command for SetVolume {
    const TEXT: &'static [u8] = b"VOL";

    type Params = SingleParam<Volume>;

    type Response = OkResponse;

    fn params(self) -> Self::Params {
        SingleParam(self.0)
    }
}
