macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

// https://lukaswirth.dev/tlborm/decl-macros/building-blocks/counting.html#repetition-with-replacement
macro_rules! count_tts {
    ($($tts:tt)*) => {0usize $(+ replace_expr!($tts 1usize))*};
}

macro_rules! get_set_command {
    // Top level - many params
    (
        text: $text:literal,
        $(#[$get_doc:meta])?
        get: $get:ident $(($query_type:tt))?,
        $(#[$set_doc:meta])?
        set: $set:ident,
        type: $set_name:ident $set_error:tt $set_fields:tt,
        $(non_program_mode: $npm:tt,)?
    ) => {
        get_set_command!(@get $get $(($query_type))? $text $set_name $($get_doc)?);
        get_set_command!(@set $set $text $set_name $($set_doc)?);
        get_set_command!(@params $set_name $set_error $set_fields);

        $(get_set_command!(@npm $get $set $npm);)?
    };
    // Top level - single param
    (
        text: $text:literal,
        $(#[$get_doc:meta])?
        get: $get:ident $(($query_type:tt))?,
        $(#[$set_doc:meta])?
        set: $set:ident,
        $(#[$field_doc:meta])?
        single_field: $kind:tt $field_type:ident $field_body:tt $field_error:ident,
        $(non_program_mode: $npm:tt,)?
    ) => {
        get_set_command!(@get $get $(($query_type))? $text $field_type $($get_doc)?);
        get_set_command!(@set_single $set $text $field_type $([$set_doc])?);
        get_set_command!(@single_param $($field_doc)? $kind $field_type $field_body $field_error);

        $(get_set_command!(@npm $get $set $npm);)?
    };
    // Param set
    (
        @params $set_name:ident ($set_error_name:ident) (
            $($(#[$field_doc:meta])? $field_name:ident: $kind:tt $type_name:ident $body:tt),+ $(,)?
        )
    ) => {
        $(get_set_command!(@param $kind $type_name $body $($field_doc)?);)*

        #[derive(Clone, Debug)]
        pub struct $set_name {
            $(pub $field_name: $type_name),*
        }

        #[doc(hidden)]
        impl IntoIterator for $set_name {
            type Item = tokio_util::bytes::Bytes;
            type IntoIter = std::array::IntoIter<Self::Item, { count_tts!($($field_name)*) }>;
            fn into_iter(self) -> Self::IntoIter {
                [
                    $(crate::command::IntoParam::into_param(self.$field_name)),*
                ].into_iter()
            }
        }

        impl crate::command::Params for $set_name {
            fn size_hint(&self) -> usize {
                let field_count = count_tts!($($field_name)*);
                let commas = field_count - 1;
                let field_sizes = [
                    $(crate::command::IntoParam::size_hint(&self.$field_name)),*
                ].into_iter().sum::<usize>();

                commas + field_sizes
            }
        }

        #[derive(Clone, Debug, thiserror::Error)]
        #[error("failed to deserialize field {0}")]
        pub struct $set_error_name(&'static str);

        impl crate::command::Response for $set_name {
            type Error = $set_error_name;

            fn deserialize<'i, I: Iterator<Item = &'i tokio_util::bytes::Bytes>>(raw_values: I) -> Result<Self, Self::Error> {
                use itertools::Itertools;
                let (
                    $($field_name,)*
                ) = raw_values.collect_tuple().expect("incorrect number of fields");
                Ok(Self {
                    $(
                        $field_name: crate::command::ResponseField::deserialize($field_name)
                            .ok_or($set_error_name(stringify!($field_name)))?,
                    )*
                })
            }

            fn expected_field_count() -> usize {
                count_tts!($($field_name)*)
            }
        }
    };
    // Single param
    (@single_param $(#[$doc:meta])? $kind:tt $name:ident $body:tt $field_error:ident) => {
        get_set_command!(@param $kind $name $body $($doc)?);

        #[derive(Clone, Debug, thiserror::Error)]
        #[error("failed to deserialize")]
        pub struct $field_error;

        impl crate::command::Response for $name {
            type Error = $field_error;


            fn deserialize<'i, I: Iterator<Item = &'i tokio_util::bytes::Bytes>>(mut raw_values: I) -> Result<Self, Self::Error> {
                if let Some(val) = raw_values.next() && raw_values.next().is_none() {
                    crate::command::ResponseField::deserialize(val)
                        .ok_or($field_error)
                } else {
                    Err($field_error)
                }
            }

            fn expected_field_count() -> usize {
                1
            }
        }
    };
    // Generators for param types
    (@param enum $name:ident { $($(#[$variant_doc:meta])? $variant:ident => $val:literal),* $(,)? } $($doc:meta)?) => {
        #[derive(Clone, Copy, Debug)]
        $(#[$doc])?
        pub enum $name { $($(#[$variant_doc])? $variant),* }

        impl crate::command::IntoParam for $name {
            fn into_param(self) -> tokio_util::bytes::Bytes {
                tokio_util::bytes::Bytes::from_static(match self {
                    $(Self::$variant => $val),*
                })
            }
            fn size_hint(&self) -> usize {
                [$($val.len()),*].into_iter().max().unwrap_or(0)
            }
        }

        impl crate::command::ResponseField for $name {
            fn deserialize(raw: &[u8]) -> Option<Self> {
                match raw {
                    $($val => Some(Self::$variant),)*
                    _ => None
                }
            }
        }
    };
    (@param range $name:ident ($range:expr => $type:ty) $($doc:meta)?) => {
        #[derive(Clone, Copy, Debug)]
        $(#[$doc])?
        pub struct $name($type);

        impl $name {
            pub fn new(value: $type) -> Option<Self> {
                ($range).contains(&value).then_some(Self(value))
            }
        }

        impl crate::command::IntoParam for $name {
            fn into_param(self) -> tokio_util::bytes::Bytes {
                tokio_util::bytes::Bytes::from(format!("{}", self.0))
            }

            fn size_hint(&self) -> usize {
                <$type as itoa::Integer>::MAX_STR_LEN
            }
        }

        impl crate::command::ResponseField for $name {
            fn deserialize(raw: &[u8]) -> Option<Self> {
                str::from_utf8(raw).ok()?
                    .parse().ok()
                    .and_then(Self::new)
            }
        }
    };
    (@param str $name:ident ($max_len:literal) $($doc:meta)?) => {
        #[derive(Clone, Debug)]
        $(#[$doc])?
        pub struct $name(Vec<u8>);

        impl $name {
            pub fn new(name: &[u8]) -> Option<Self> {
                (name.len() <= $max_len)
                    .then_some(Self(Vec::from(name)))
            }
        }

        impl crate::command::IntoParam for $name {
            fn into_param(self) -> tokio_util::bytes::Bytes {
                self.0.into()
            }

            fn size_hint(&self) -> usize { self.0.len() }
        }

        impl crate::command::ResponseField for $name {
            fn deserialize(raw: &[u8]) -> Option<Self> {
                Some(Self(Vec::from(raw)))
            }
        }
    };
    // Commands
    (@get $name:ident $text:literal $response:ident $($doc:meta)?) => {
        #[derive(Clone, Copy, Debug)]
        $(#[$doc])?
        pub struct $name;
        impl crate::command::Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = crate::command::NoParams;
            type Response = $response;
            fn params(self) -> Self::Params { crate::command::NoParams }
        }
    };
    (@get $name:ident ($query_type:ty) $text:literal $response:ident $($doc:meta)?) => {
        #[derive(Clone, Debug)]
        $(#[$doc])?
        pub struct $name(pub $query_type);
        impl crate::command::Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = crate::command::single_param::SingleParam<$query_type>;
            type Response = $response;
            fn params(self) -> Self::Params {
                crate::command::single_param::SingleParam(self.0)
            }
        }
    };
    (@set $name:ident $text:literal $params:ident $($doc:meta)?) => {
        #[derive(Clone, Debug)]
        $(#[$doc])?
        pub struct $name(pub $params);
        impl crate::command::Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = $params;
            type Response = crate::command::OkResponse;
            fn params(self) -> Self::Params { self.0 }
        }
    };
    (@set_single $name:ident $text:literal $param:ident $($doc:meta)?) => {
        #[derive(Clone, Debug)]
        $(#[$doc])?
        pub struct $name(pub $param);
        impl crate::command::Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = crate::command::SingleParam<$param>;
            type Response = crate::command::OkResponse;
            fn params(self) -> Self::Params { crate::command::SingleParam(self.0) }
        }
    };
    // Non-program mode impl
    (@npm $get:ident $set:ident $_npm:tt) => {
        impl crate::command::NonProgramModeCommand for $get {}
        impl crate::command::NonProgramModeCommand for $set {}
    };
}
