use crate::command::{Param, ParamSet};

pub struct NoParams;

impl IntoIterator for NoParams {
    type Item = &'static dyn Param;
    type IntoIter = std::iter::Empty<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl ParamSet<'static> for NoParams {
    fn count(&self) -> usize {
        0
    }

    fn size(&self) -> usize {
        0
    }
}
