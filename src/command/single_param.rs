use tokio_util::bytes::Bytes;

use crate::command::{IntoParam, Params};

pub struct SingleParam<T: IntoParam>(pub T);

impl<T: IntoParam> IntoIterator for SingleParam<T> {
    type Item = Bytes;

    type IntoIter = std::iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.0.into_param())
    }
}

impl<T: IntoParam> Params for SingleParam<T> {
    fn size_hint(&self) -> usize {
        1
    }
}
