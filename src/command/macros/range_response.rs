macro_rules! range_response {
    ($name:ty => $error:ident : $invalid_variant:ident($msg:literal)) => {
        #[derive(::core::fmt::Debug, ::thiserror::Error)]
        pub enum $error {
            #[error(transparent)]
            Utf8Error(#[from] ::core::str::Utf8Error),
            #[error(transparent)]
            Parse(#[from] ::core::num::ParseIntError),
            #[error($msg)]
            $invalid_variant(u8),
        }

        impl crate::command::Response for $name {
            type Error = $error;

            fn deserialize(
                raw_values: &[::tokio_util::bytes::Bytes],
            ) -> ::core::result::Result<Self, Self::Error> {
                let level = ::core::str::from_utf8(&raw_values[0])?.parse()?;
                Self::new(level).ok_or(Self::Error::$invalid_variant(level))
            }
            fn expected_field_count() -> usize {
                1
            }
        }
    };
}

pub(crate) use range_response;

#[cfg(test)]
mod tests {
    use claims::assert_matches;

    use crate::command::{Response, range_param, test::deserialize};

    range_param!(U8RangeParam(0..=10): u8);
    range_response!(U8RangeParam => ParamError : Invalid("invalid"));

    #[test]
    fn expected_field_count() {
        assert_eq!(U8RangeParam::expected_field_count(), 1);
    }

    #[test]
    fn deserializes_valid_response() {
        let response = deserialize::<U8RangeParam, _>(b"0").unwrap();
        assert_eq!(response, U8RangeParam(0));
    }

    #[test]
    fn invalid_utf8() {
        let err = deserialize::<U8RangeParam, _>(&[0, 159]).unwrap_err();
        assert_matches!(err, ParamError::Utf8Error(_));
    }

    #[test]
    fn invalid_integer() {
        let err = deserialize::<U8RangeParam, _>(b"a").unwrap_err();
        assert_matches!(err, ParamError::Parse(_));
    }

    #[test]
    fn out_of_range() {
        let err = deserialize::<U8RangeParam, _>(b"20").unwrap_err();
        assert_matches!(err, ParamError::Invalid(20));
    }
}
