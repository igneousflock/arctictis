macro_rules! string_response {
    ($name:ident => $error:ident) => {
        #[derive(::core::fmt::Debug, ::thiserror::Error)]
        #[error("invalid UTF-8 bytes")]
        pub struct $error(#[from] pub ::core::str::Utf8Error);

        #[derive(
            ::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::PartialEq, ::core::cmp::Eq,
        )]
        pub struct $name(pub ::std::string::String);

        impl crate::command::Response for $name {
            type Error = $error;

            fn deserialize(
                raw_values: &[::tokio_util::bytes::Bytes],
            ) -> ::core::result::Result<Self, Self::Error> {
                let val = ::core::str::from_utf8(&raw_values[0])?.to_string();
                Ok(Self(val))
            }

            fn expected_field_count() -> usize {
                1
            }
        }
    };
}

pub(crate) use string_response;

#[cfg(test)]
mod tests {
    use claims::assert_matches;

    use crate::command::{Response, test::deserialize};

    string_response!(StringResponse => StringResponseError);

    #[test]
    fn expected_field_count() {
        assert_eq!(StringResponse::expected_field_count(), 1);
    }

    #[test]
    fn deserializes_valid_response() {
        let response = deserialize::<StringResponse, _>(b"hello, world!").unwrap();
        assert_eq!(response, StringResponse("hello, world!".to_string()));
    }

    #[test]
    fn invalid_utf8() {
        let err = deserialize::<StringResponse, _>(&[0, 159]).unwrap_err();
        assert_matches!(err, StringResponseError(_));
    }
}

