macro_rules! command {
    ($text:literal: $name:ident) => {
        pub struct $name;

        impl Command<'static> for $name {
            const TEXT: &'static [u8] = $text;
            type Params = NoParams;
            type Response = OkResponse;

            fn param_set(&self) -> Self::Params {
                NoParams
            }
        }
    };
    ($text:literal: $name:ident => $response:ident) => {
        pub struct $name;

        impl Command<'static> for $name {
            const TEXT: &'static [u8] = $text;
            type Params = NoParams;
            type Response = $response;

            fn param_set(&self) -> Self::Params {
                NoParams
            }
        }
    };
}

pub(crate) use command;
