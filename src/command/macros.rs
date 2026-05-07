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
            fn total_size(&self) -> usize {
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
