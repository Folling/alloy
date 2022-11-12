pub trait EnumCount {
    fn count() -> usize;
}

pub trait EnumIter<I: Iterator + DoubleEndedIterator + ExactSizeIterator> {
    fn iter() -> I;
}
