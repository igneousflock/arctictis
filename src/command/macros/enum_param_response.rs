macro_rules! enum_param_response {
    ($name:ident { $($variant:ident => $val:literal),+ $(,)? } : $error:ident($msg:literal)) => {
        #[derive(
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::core::fmt::Debug,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
        )]
        pub enum $name {
            $($variant),+
        }

        impl crate::command::Params for $name {
            fn count(&self) -> usize {
                1
            }

            fn max_size(&self) -> usize {
                2
            }

            fn serialize_to(&self, mut buffer: crate::command::ParamBuffer) {
                let txt = match self {
                    $(Self::$variant => $val,)+
                };
                buffer.serialize_param(txt);
            }
        }

        #[derive(
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::core::fmt::Debug,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
            ::thiserror::Error,
        )]
        #[error($msg)]
        pub struct $error;

        impl crate::command::Response for $name {
            type Error = $error;

            fn deserialize(raw_values: &[::tokio_util::bytes::Bytes]) -> ::core::result::Result<Self, Self::Error> {
                let val = match raw_values[0].as_ref() {
                    $($val => Self::$variant,)+
                    _ => return ::core::result::Result::Err($error),
                };
                Ok(val)
            }


            fn expected_field_count() -> usize {
                1
            }
        }
    };
}

pub(crate) use enum_param_response;
