macro_rules! command {
    ($text:literal: $name:ident) => {
        pub struct $name;

        impl crate::command::Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = crate::command::NoParams;
            type Response = crate::command::OkResponse;

            command!(@no_params_fn);
        }
    };
    ($text:literal: $name:ident => $response:ty) => {
        pub struct $name;

        impl crate::command::Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = crate::command::NoParams;
            type Response = $response;

            command!(@no_params_fn);
        }
    };
    ($text:literal: $name:ident($param_set:ty) => $response:ty) => {
        pub struct $name(pub $param_set);

        impl crate::command::Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = $param_set;
            type Response = $response;

            fn params(&self) -> &Self::Params {
                &self.0
            }
        }
    };
    (@no_params_fn) => {
        fn params(&self) -> &crate::command::NoParams {
            &crate::command::NoParams
        }
    };
}

macro_rules! range_param {
    ($name:ident($range:expr): $type:ty) => {
        #[derive(
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::core::fmt::Debug,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
        )]
        pub struct $name($type);

        impl $name {
            pub fn new(value: $type) -> ::core::option::Option<Self> {
                ($range).contains(&value).then_some(Self(value))
            }

            #[allow(unused)]
            pub fn value(&self) -> $type {
                self.0
            }
        }

        impl crate::command::Params for $name {
            fn count(&self) -> usize {
                1
            }
            fn max_size(&self) -> usize {
                <$type as ::itoa::Integer>::MAX_STR_LEN
            }
            fn serialize_to(&self, mut buffer: crate::command::ParamBuffer) {
                let mut serialized = ::itoa::Buffer::new();
                buffer.serialize_param(serialized.format(self.0).as_bytes());
            }
        }
    };
}

macro_rules! range_response {
    ($name:ty => $error:ty : $variant:ident) => {
        impl crate::command::Response for $name {
            type Error = $error;

            fn deserialize(
                raw_values: &[::tokio_util::bytes::Bytes],
            ) -> ::core::result::Result<Self, Self::Error> {
                let level = ::core::str::from_utf8(&raw_values[0])?.parse()?;
                Self::new(level).ok_or(Self::Error::$variant(level))
            }
            fn expected_field_count() -> usize {
                1
            }
        }
    };
}

pub(crate) use command;
pub(crate) use range_param;
pub(crate) use range_response;

#[cfg(test)]
mod tests {
    use claims::{assert_none, assert_some_eq};
    use tokio_util::bytes::BytesMut;

    use crate::{
        Command, OkResponse,
        command::{NoParams, ParamBuffer, Params, Response},
    };

    mod command {
        use super::*;

        #[derive(Debug, thiserror::Error)]
        pub enum ParamError {
            #[error("invalid UTF-8 bytes")]
            Utf8Error(#[from] std::str::Utf8Error),
            #[error(transparent)]
            Parse(#[from] std::num::ParseIntError),
            #[error("invalid value, got `{0}`")]
            Invalid(u8),
        }

        range_param!(RangeParam(0..=15): u8);
        range_response!(RangeParam => ParamError : Invalid);

        #[test]
        fn no_params_ok_response() {
            command!(b"CMD": SimpleCommand);
            assert_generated_command(&SimpleCommand, b"CMD", &NoParams);
        }

        #[test]
        fn no_params_single_value_response() {
            command!(b"CMD": ResponseCommand => RangeParam);

            assert_generated_command(&ResponseCommand, b"CMD", &NoParams);
        }

        #[test]
        fn range_param_ok_response() {
            command!(b"CMD": ParamCommand(RangeParam) => OkResponse);

            let param = RangeParam(0);
            assert_generated_command(&ParamCommand(param), b"CMD", &param);
        }

        fn assert_generated_command<C: Command>(command: &C, text: &[u8], params: &C::Params)
        where
            C::Params: std::fmt::Debug + PartialEq,
        {
            assert_eq!(C::TEXT, text);
            assert_eq!(command.params(), params);
        }
    }

    mod range_param {

        use super::*;

        range_param!(U8RangeParam(0..=10): u8);

        #[test]
        fn produces_value() {
            for i in 0..=10 {
                assert_some_eq!(U8RangeParam::new(i), U8RangeParam(i));
            }
        }

        #[test]
        fn produces_none_for_invalid_value() {
            assert_none!(U8RangeParam::new(20));
        }

        #[test]
        fn returns_value() {
            assert_eq!(U8RangeParam(10).value(), 10);
        }

        #[test]
        fn generated_param_impl() {
            assert_eq!(U8RangeParam(0).count(), 1);
            assert_eq!(U8RangeParam(0).max_size(), 3);

            for i in 0..=10 {
                let param = U8RangeParam(i);

                let mut bytes = BytesMut::new();
                let buf = ParamBuffer::new(&mut bytes);

                param.serialize_to(buf);

                assert_eq!(bytes, format!(",{i}").as_bytes());
            }
        }
    }

    mod range_response {
        use claims::assert_matches;
        use tokio_util::bytes::Bytes;

        use super::*;

        #[derive(Debug, thiserror::Error)]
        pub enum ParamError {
            #[error("invalid UTF-8 bytes")]
            Utf8Error(#[from] std::str::Utf8Error),
            #[error(transparent)]
            Parse(#[from] std::num::ParseIntError),
            #[error("invalid value, got `{0}`")]
            Invalid(u8),
        }

        range_param!(U8RangeParam(0..=10): u8);
        range_response!(U8RangeParam => ParamError : Invalid);

        #[test]
        fn expected_field_count() {
            assert_eq!(U8RangeParam::expected_field_count(), 1);
        }

        #[test]
        fn deserializes_valid_response() {
            let response = deserialize(b"0").unwrap();
            assert_eq!(response, U8RangeParam(0));
        }

        #[test]
        fn invalid_utf8() {
            let err = deserialize(&[0, 159]).unwrap_err();
            assert_matches!(err, ParamError::Utf8Error(_));
        }

        #[test]
        fn invalid_integer() {
            let err = deserialize(b"a").unwrap_err();
            assert_matches!(err, ParamError::Parse(_));
        }

        #[test]
        fn out_of_range() {
            let err = deserialize(b"20").unwrap_err();
            assert_matches!(err, ParamError::Invalid(20));
        }

        fn deserialize<T: AsRef<[u8]>>(val: &'static T) -> Result<U8RangeParam, ParamError> {
            U8RangeParam::deserialize(&[Bytes::from(val.as_ref())])
        }
    }
}
