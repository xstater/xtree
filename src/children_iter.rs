use crate::Tree;

/// The immutable iterator over all the children
pub struct ChildrenIter<'a,T>{
    pub (in crate) raw_iter : std::slice::Iter<'a, Tree<T>>
}

impl<'a,T> Iterator for ChildrenIter<'a,T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw_iter.next().map(|v| &v.data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw_iter.size_hint()
    }

    fn count(self) -> usize where
        Self: Sized, {
        self.raw_iter.count()
    }

    fn last(self) -> Option<Self::Item> where
        Self: Sized, {
        self.raw_iter.last().map(|v| &v.data)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.raw_iter.nth(n).map(|v| &v.data)
    }
}

/// The mutable iterator over all the children
pub struct ChildrenIterMut<'a,T>{
    pub (in crate) raw_iter : std::slice::IterMut<'a, Tree<T>>
}

impl<'a,T> Iterator for ChildrenIterMut<'a,T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw_iter.next().map(|v| &mut v.data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw_iter.size_hint()
    }

    fn count(self) -> usize where
        Self: Sized, {
        self.raw_iter.count()
    }

    fn last(self) -> Option<Self::Item> where
        Self: Sized, {
        self.raw_iter.last().map(|v| &mut v.data)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.raw_iter.nth(n).map(|v| &mut v.data)
    }
}
