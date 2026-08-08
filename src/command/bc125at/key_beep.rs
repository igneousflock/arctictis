use itertools::Itertools;
use tokio_util::bytes::Bytes;

use crate::{
    Command, OkResponse,
    command::{IntoParam, NoParams, Params, Response, ResponseField},
};

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum KeyBeepSettingError {
    #[error("invalid beep level")]
    BeepLevel,
    #[error("invalid key lock status")]
    KeyLockStatus,
    #[error("expected two fields")]
    Malformed,
}

pub enum BeepLevel {
    Auto,
    Off,
}

impl IntoParam for BeepLevel {
    fn into_param(self) -> Bytes {
        match self {
            BeepLevel::Auto => b"0".as_ref(),
            BeepLevel::Off => b"99".as_ref(),
        }
        .into()
    }
}

impl ResponseField for BeepLevel {
    fn deserialize(raw: &[u8]) -> Option<Self> {
        match raw {
            b"0" => Some(Self::Auto),
            b"99" => Some(Self::Off),
            _ => None,
        }
    }
}

pub enum KeyLockStatus {
    Off,
    On,
}

impl IntoParam for KeyLockStatus {
    fn into_param(self) -> Bytes {
        match self {
            KeyLockStatus::Off => b"0".as_ref(),
            KeyLockStatus::On => b"1".as_ref(),
        }
        .into()
    }
}

impl ResponseField for KeyLockStatus {
    fn deserialize(raw: &[u8]) -> Option<Self> {
        match raw {
            b"0" => Some(Self::Off),
            b"1" => Some(Self::On),
            _ => None,
        }
    }
}

pub struct KeyBeepSetting(BeepLevel, KeyLockStatus);

impl IntoIterator for KeyBeepSetting {
    type Item = Bytes;

    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0.into_param(), self.1.into_param()].into_iter()
    }
}

impl Params for KeyBeepSetting {
    fn size_hint(&self) -> usize {
        1 // commas
            + 2 // beep level
            + 1 // key lock status
    }
}

impl Response for KeyBeepSetting {
    type Error = KeyBeepSettingError;

    fn deserialize<'i, I: Iterator<Item = &'i Bytes>>(raw_values: I) -> Result<Self, Self::Error> {
        let (bl, kls) = raw_values.collect_tuple().ok_or(Self::Error::Malformed)?;
        let bl = ResponseField::deserialize(bl).ok_or(Self::Error::BeepLevel)?;
        let kls = ResponseField::deserialize(kls).ok_or(Self::Error::KeyLockStatus)?;
        Ok(Self(bl, kls))
    }

    fn expected_field_count() -> usize {
        2
    }
}

pub struct GetKeyBeepSetting;

impl Command for GetKeyBeepSetting {
    const TEXT: &'static [u8] = b"KBP";

    type Params = NoParams;

    type Response = KeyBeepSetting;

    fn params(self) -> Self::Params {
        NoParams
    }
}

pub struct SetKeyBeep(KeyBeepSetting);

impl Command for SetKeyBeep {
    const TEXT: &'static [u8] = b"KBP";

    type Params = KeyBeepSetting;

    type Response = OkResponse;

    fn params(self) -> Self::Params {
        self.0
    }
}
