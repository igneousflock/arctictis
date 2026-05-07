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

#[cfg(test)]
mod tests {

    use crate::{
        Command, OkResponse,
        command::{NoParams, range_param, range_response},
    };

    range_param!(RangeParam(0..=15): u8);
    range_response!(RangeParam => ParamError : Invalid("invalid"));

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
