// #[macro_export]
// #[doc(hidden)]
macro_rules! gen_command {
    ($cmd:literal, $name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name;

        gen_command!(@traits $cmd, $name);

        impl Command for $name {
            fn as_bytes(&self) -> &'static [u8] { $cmd }
        }
    };
    ($cmd:literal, $name: ident ( $param_ty:ty )) => {
        #[derive(Clone, Debug)]
        pub struct $name(pub $param_ty);

        gen_command!(@traits $cmd, $name);

        impl Command for $name {
            fn as_bytes(&self) -> &'static [u8] { $cmd }
            fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
                [&self.0 as &dyn Param]
            }
        }
    };
    ($cmd:literal, $name: ident { $($param:ident: $param_ty:ty),+ }) => {
        #[derive(Clone, Debug)]
        pub struct $name { $(pub $param: $param_ty),+ }

        gen_command!(@traits $cmd, $name);

        impl Command for $name {
            fn as_bytes(&self) -> &'static [u8] { $cmd }
            fn params(&self) -> impl IntoIterator<Item = &dyn Param> {
                [$(&self.$param as &dyn Param),+]
            }
        }
    };
    (@traits $cmd:literal, $name:ident) => {
        impl private::Sealed for $name {}
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                String::from_utf8_lossy($cmd).fmt(f)
            }
        }
    };
}

// #[macro_export]
// #[doc(hidden)]
macro_rules! gen_param {
    (pub enum $name:ident { $($variant:ident => $val:literal),+ $(,)? }) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            $($variant),+
        }

        gen_param!(@traits $name);

        impl Param for $name {
            fn write_bytes(&self, dst: &mut tokio_util::bytes::BytesMut) {
                let s = match self {
                    $(Self::$variant => $val.as_slice()),+
                };
                dst.extend_from_slice(s);
            }
        }
    };
    (pub range $name:ident ($range:expr)) => {
        #[derive(Clone, Debug)]
        pub struct $name(u8);

        gen_param!(@traits $name);

        impl $name {
            pub fn new(value: u8) -> Self {
                assert!($range.contains(&value));
                Self(value)
            }
        }
        impl Param for $name {
            fn write_bytes(&self, dst: &mut tokio_util::bytes::BytesMut) {
                self.0.write_bytes(dst);
            }
        }
    };
    (@traits $name:ident) => {
        impl private::Sealed for $name {}
    };
}
