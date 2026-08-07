use crate::command::Params;

pub struct NoParams;

impl IntoIterator for NoParams {
    type Item = &'static [u8];

    type IntoIter = std::iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl Params for NoParams {
    fn size_hint(&self) -> usize {
        0
    }
}
