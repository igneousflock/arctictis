use crate::command::{ParamBuffer, Params};

pub struct NoParams;

impl Params for NoParams {
    fn count(&self) -> usize {
        0
    }

    fn total_size(&self) -> usize {
        0
    }

    fn serialize_to(&self, _buffer: ParamBuffer) {
        /* noop */
    }
}
