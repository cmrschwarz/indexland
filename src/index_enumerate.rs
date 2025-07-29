use core::marker::PhantomData;

use crate::Idx;

pub struct IndexEnumerate<I, BaseIter> {
    next_idx: usize,
    base_iter: BaseIter,
    _phantom: PhantomData<I>,
}

impl<I, BaseIter: Iterator> IndexEnumerate<I, BaseIter> {
    pub fn new<IntoBaseIter: IntoIterator<IntoIter = BaseIter>>(
        next_idx: I,
        base_iter: IntoBaseIter,
    ) -> Self
    where
        I: Idx,
    {
        Self {
            next_idx: next_idx.into_usize(),
            base_iter: base_iter.into_iter(),
            _phantom: PhantomData,
        }
    }
}

impl<I, It: Iterator> Iterator for IndexEnumerate<I, It>
where
    I: Idx,
{
    type Item = (I, It::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.base_iter.next()?;
        let idx = self.next_idx;
        self.next_idx = idx + 1;
        Some((I::from_usize(idx), value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base_iter.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.base_iter.count()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.base_iter.nth(n) {
            Some(v) => {
                let pos = self.next_idx;
                self.next_idx = pos + 1;
                Some((I::from_usize(pos.into_usize() + n), v))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "std")]
    #[test]
    fn no_overflow_in_enum() {
        use crate::enum_index_array;
        use indexland_derive::Idx;
        use std::vec::Vec;

        #[derive(Idx)]
        enum Foo {
            A,
            B,
            C,
        }

        let arr = enum_index_array![
            Foo::A => 0,
            Foo::B => 1,
            Foo::C => 2,
        ];

        assert_eq!(
            arr.into_iter_enumerated().collect::<Vec<_>>(),
            [0, 1, 2].map(|x| (x, x))
        );
    }
}
