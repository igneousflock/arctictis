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
    // Top level
    (
        text: $text:literal,
        $(#[$get_doc:meta])?
        get: $get:ident,
        $(#[$set_doc:meta])?
        set: $set:ident,
        type: $set_name:ident $set_error:tt $set_fields:tt,
    ) => {
        get_set_command!(@get $get $text $set_name $($get_doc)?);
        get_set_command!(@set $set $text $set_name $($set_doc)?);
        get_set_command!(@params $set_name $set_error $set_fields);
    };
    // Param set
    (
        @params $set_name:ident ($set_error_name:ident) (
            $($(#[$field_doc:meta])? $field_name:ident: $kind:tt $type_name:ident $body:tt),* $(,)?
        )
    ) => {
        $(get_set_command!(@param $kind $type_name $body $($field_doc)?);)*

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

        #[derive(Debug, thiserror::Error)]
        pub enum $set_error_name {
            #[error("failed to deserialize field {0}")]
            BadField(&'static str),
            #[error("incorrect number of fields")]
            Malformed,
        }

        impl crate::command::Response for $set_name {
            type Error = $set_error_name;

            fn deserialize<'i, I: Iterator<Item = &'i tokio_util::bytes::Bytes>>(raw_values: I) -> Result<Self, Self::Error> {
                use itertools::Itertools;
                let (
                    $($field_name,)*
                ) = raw_values.collect_tuple().ok_or(Self::Error::Malformed)?;
                Ok(Self {
                    $(
                        $field_name: crate::command::ResponseField::deserialize($field_name)
                            .ok_or(Self::Error::BadField(stringify!($field_name)))?,
                    )*
                })
            }

            fn expected_field_count() -> usize {
                2
            }
        }
    };
    // Generators for param types
    (@param enum $name:ident {$($(#[$variant_doc:meta])? $variant:ident => $val:literal),* $(,)? } $($doc:meta)?) => {
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
    // Commands
    (@get $name:ident $text:literal $response:ident $($doc:meta)?) => {
        $(#[$doc])?
        pub struct $name;
        impl crate::command::Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = crate::command::NoParams;
            type Response = $response;
            fn params(self) -> Self::Params { crate::command::NoParams }
        }
    };
    (@set $name:ident $text:literal $params:ident $($doc:meta)?) => {
        $(#[$doc])?
        pub struct $name(pub $params);
        impl crate::command::Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = $params;
            type Response = crate::command::OkResponse;
            fn params(self) -> Self::Params { self.0 }
        }
    }
}
