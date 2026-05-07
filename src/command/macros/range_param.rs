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

pub(crate) use range_param;

#[cfg(test)]
mod tests {
    use claims::{assert_none, assert_some_eq};
    use tokio_util::bytes::BytesMut;

    use crate::command::{ParamBuffer, Params};

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
