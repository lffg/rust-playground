use std::iter::Peekable;

pub struct IterWithLast<I: Iterator>(Peekable<I>);

impl<I: Iterator> Iterator for IterWithLast<I> {
    type Item = (bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|val| (self.0.peek().is_none(), val))
    }
}

pub trait WithLast: Iterator + Sized {
    fn with_last(self) -> IterWithLast<Self>;
}

impl<I: Iterator> WithLast for I {
    fn with_last(self) -> IterWithLast<Self> {
        IterWithLast(self.peekable())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::WithLast;

        let mut iter = "abc".chars().with_last();
        assert_eq!(iter.next().unwrap(), (false, 'a'));
        assert_eq!(iter.next().unwrap(), (false, 'b'));
        assert_eq!(iter.next().unwrap(), (true, 'c'));
    }
}
