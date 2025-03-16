use crate::Idx;

pub struct IndexEnumerate<I, BaseIter> {
    pos: I,
    base_iter: BaseIter,
}

impl<I, BaseIter: Iterator> IndexEnumerate<I, BaseIter> {
    pub fn new<IntoBaseIter: IntoIterator<IntoIter = BaseIter>>(
        pos: I,
        base_iter: IntoBaseIter,
    ) -> Self {
        Self {
            pos,
            base_iter: base_iter.into_iter(),
        }
    }
}

impl<I, IT: Iterator> Iterator for IndexEnumerate<I, IT>
where
    I: Idx,
{
    type Item = (I, IT::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.base_iter.next()?;
        let idx = self.pos;
        self.pos = I::from_usize(self.pos.into_usize() + 1);
        Some((idx, value))
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
                let pos = self.pos;
                self.pos += I::from_usize(n + 1);
                Some((pos + I::from_usize(n), v))
            }
            None => None,
        }
    }
}
