use tokio_util::bytes::{Buf, Bytes};

pub struct BytesSplit(Bytes, u8);

impl BytesSplit {
    pub fn new(inner: Bytes, split_at: u8) -> Self {
        Self(inner, split_at)
    }
}

impl Iterator for BytesSplit {
    type Item = Bytes;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        // find the index of the first delimiter
        let Some(i) = self
            .0
            .iter()
            .enumerate()
            .find_map(|(i, b)| (*b == self.1).then_some(i))
        else {
            // we're on the last element
            let last_elem = self.0.clone();
            self.0.clear();
            return Some(last_elem);
        };

        // extract the element
        let elem = self.0.split_to(i);
        // remove the comma
        self.0.advance(1);

        Some(elem)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use tokio_util::bytes::Bytes;

    #[test]
    fn splits_on_commas() {
        let bytes = Bytes::from(b"foo,bar,baz".as_slice());
        let mut split = BytesSplit::new(bytes, b',');

        assert_eq!(split.next().unwrap().as_ref(), b"foo");
        assert_eq!(split.next().unwrap().as_ref(), b"bar");
        assert_eq!(split.next().unwrap().as_ref(), b"baz");
        assert_eq!(split.next(), None);
    }

    #[test]
    fn trailing_comma_returns_none() {
        let bytes = Bytes::from(b"foo,".as_slice());
        let mut split = BytesSplit::new(bytes, b',');

        assert_eq!(split.next().unwrap().as_ref(), b"foo");
        assert_eq!(split.next(), None);
    }

    #[test]
    fn single_comma_returns_empty_then_none() {
        let bytes = Bytes::from(b",".as_slice());
        let mut split = BytesSplit::new(bytes, b',');

        assert_eq!(split.next().unwrap().as_ref(), b"");
        assert_eq!(split.next(), None);
    }

    #[test]
    fn empty_string_returns_none() {
        let bytes = Bytes::from(b"".as_slice());
        let mut split = BytesSplit::new(bytes, b',');

        assert_eq!(split.next(), None);
    }
}
