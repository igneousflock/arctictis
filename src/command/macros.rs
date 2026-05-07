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

pub(crate) use command;
