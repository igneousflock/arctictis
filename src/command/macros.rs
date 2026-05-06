macro_rules! command {
    ($text:literal: $name:ident) => {
        pub struct $name;

        impl Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = NoParams;
            type Response = OkResponse;

            fn params(&self) -> &Self::Params {
                &NoParams
            }
        }
    };
    ($text:literal: $name:ident => $response:ident) => {
        pub struct $name;

        impl Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = NoParams;
            type Response = $response;

            fn params(&self) -> &Self::Params {
                &NoParams
            }
        }
    };
    ($text:literal: $name:ident($param_set:ident) => $response:ident) => {
        pub struct $name(pub $param_set);

        impl Command for $name {
            const TEXT: &'static [u8] = $text;
            type Params = $param_set;
            type Response = $response;

            fn params(&self) -> &Self::Params {
                &self.0
            }
        }
    };
}

pub(crate) use command;
