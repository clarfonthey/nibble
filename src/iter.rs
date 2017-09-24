//! Various iterators for slices of nibbles.
use core::slice;
use base::u4;
use pair::{Iter, IterMut, U4Cell, u4x2};

/// Iterator over pairs of nibbles in a slice.
#[derive(Debug)]
pub struct NibblePairs<'a> {
    inner: slice::Iter<'a, u4x2>,
}
impl<'a> NibblePairs<'a> {
    pub(crate) fn new(inner: slice::Iter<'a, u4x2>) -> Self {
        NibblePairs { inner }
    }
    pub(crate) fn as_slice(&self) -> &'a [u4x2] {
        self.inner.as_slice()
    }
}
impl<'a> Iterator for NibblePairs<'a> {
    type Item = &'a u4x2;
    fn next(&mut self) -> Option<&'a u4x2> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    fn count(self) -> usize {
        self.inner.count()
    }
    fn nth(&mut self, n: usize) -> Option<&'a u4x2> {
        self.inner.nth(n)
    }
    fn last(self) -> Option<&'a u4x2> {
        self.inner.last()
    }
    fn all<F: FnMut(&'a u4x2) -> bool>(&mut self, predicate: F) -> bool {
        self.inner.all(predicate)
    }
    fn any<F: FnMut(&'a u4x2) -> bool>(&mut self, predicate: F) -> bool {
        self.inner.any(predicate)
    }
    fn find<F: FnMut(&&'a u4x2) -> bool>(&mut self, predicate: F) -> Option<&'a u4x2> {
        self.inner.find(predicate)
    }
    fn position<F: FnMut(&'a u4x2) -> bool>(&mut self, predicate: F) -> Option<usize> {
        self.inner.position(predicate)
    }
    fn rposition<F: FnMut(Self::Item) -> bool>(&mut self, predicate: F) -> Option<usize> {
        self.inner.rposition(predicate)
    }
}
impl<'a> DoubleEndedIterator for NibblePairs<'a> {
    fn next_back(&mut self) -> Option<&'a u4x2> {
        self.inner.next_back()
    }
}
impl<'a> ExactSizeIterator for NibblePairs<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Mutable iterator over pairs of nibbles in a slice.
#[derive(Debug)]
pub struct NibblePairsMut<'a> {
    inner: slice::IterMut<'a, u4x2>,
}
impl<'a> NibblePairsMut<'a> {
    pub(crate) fn new(inner: slice::IterMut<'a, u4x2>) -> Self {
        NibblePairsMut { inner }
    }
    pub(crate) fn into_slice(self) -> &'a mut [u4x2] {
        self.inner.into_slice()
    }
}
impl<'a> Iterator for NibblePairsMut<'a> {
    type Item = &'a mut u4x2;
    fn next(&mut self) -> Option<&'a mut u4x2> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    fn count(self) -> usize {
        self.inner.count()
    }
    fn nth(&mut self, n: usize) -> Option<&'a mut u4x2> {
        self.inner.nth(n)
    }
    fn last(self) -> Option<&'a mut u4x2> {
        self.inner.last()
    }
    fn all<F: FnMut(&'a mut u4x2) -> bool>(&mut self, predicate: F) -> bool {
        self.inner.all(predicate)
    }
    fn any<F: FnMut(&'a mut u4x2) -> bool>(&mut self, predicate: F) -> bool {
        self.inner.any(predicate)
    }
    fn find<F: FnMut(&&'a mut u4x2) -> bool>(&mut self, predicate: F) -> Option<&'a mut u4x2> {
        self.inner.find(predicate)
    }
    fn position<F: FnMut(&'a mut u4x2) -> bool>(&mut self, predicate: F) -> Option<usize> {
        self.inner.position(predicate)
    }
    fn rposition<F: FnMut(Self::Item) -> bool>(&mut self, predicate: F) -> Option<usize> {
        self.inner.rposition(predicate)
    }
}
impl<'a> DoubleEndedIterator for NibblePairsMut<'a> {
    fn next_back(&mut self) -> Option<&'a mut u4x2> {
        self.inner.next_back()
    }
}
impl<'a> ExactSizeIterator for NibblePairsMut<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Iterator over nibbles in a slice.
#[derive(Debug)]
pub struct Nibbles<'a> {
    pairs: NibblePairs<'a>,
    front: Option<Iter<'a>>,
    back: Option<Iter<'a>>,
}
impl<'a> Nibbles<'a> {
    #[inline]
    pub(crate) fn new(pairs: NibblePairs<'a>, has_left_hi: bool, has_right_lo: bool) -> Self {
        let mut iter = Nibbles { pairs, front: None, back: None };
        if !has_left_hi {
            iter.next();
        }
        if !has_right_lo {
            iter.next_back();
        }
        iter
    }
}
impl<'a> Iterator for Nibbles<'a> {
    type Item = &'a u4;
    fn next(&mut self) -> Option<&'a u4> {
        // taken directly from core::iter::FlatMap source;
        // we can't use FlatMap because it doesn't have an ExactSizeIterator implementation
        loop {
            if let Some(ref mut inner) = self.front {
                if let Some(x) = inner.next() {
                    return Some(x);
                }
            }
            match self.pairs.next() {
                None => return self.back.as_mut().and_then(|it| it.next()),
                next => self.front = next.map(u4x2::iter),
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<'a> DoubleEndedIterator for Nibbles<'a> {
    fn next_back(&mut self) -> Option<&'a u4> {
        // taken directly from core::iter::FlatMap source;
        // we can't use FlatMap because it doesn't have an ExactSizeIterator implementation
        loop {
            if let Some(ref mut inner) = self.back {
                if let Some(x) = inner.next_back() {
                    return Some(x);
                }
            }
            match self.pairs.next_back() {
                None => return self.front.as_mut().and_then(|it| it.next_back()),
                next => self.back = next.map(u4x2::iter),
            }
        }
    }
}
impl<'a> ExactSizeIterator for Nibbles<'a> {
    fn len(&self) -> usize {
        let front = self.front.as_ref().map(ExactSizeIterator::len).unwrap_or(0);
        let back = self.back.as_ref().map(ExactSizeIterator::len).unwrap_or(0);
        let middle = self.pairs.len() >> 1;
        front + middle + back
    }
}

/// Mutable iterator over nibbles in a slice.
///
/// To ensure that nibbles from the same byte do not overwrite each other, this method will
/// return `U4Cell`s instead of mutable references.
#[derive(Debug)]
pub struct NibblesMut<'a> {
    pairs: NibblePairsMut<'a>,
    front: Option<IterMut<'a>>,
    back: Option<IterMut<'a>>,
}
impl<'a> NibblesMut<'a> {
    #[inline]
    pub(crate) fn new(pairs: NibblePairsMut<'a>, has_left_hi: bool, has_right_lo: bool) -> Self {
        let mut iter = NibblesMut { pairs, front: None, back: None };
        if !has_left_hi {
            iter.next();
        }
        if !has_right_lo {
            iter.next_back();
        }
        iter
    }
}
impl<'a> Iterator for NibblesMut<'a> {
    type Item = &'a U4Cell;
    fn next(&mut self) -> Option<&'a U4Cell> {
        // taken directly from core::iter::FlatMap source;
        // we can't use FlatMap because it doesn't have an ExactSizeIterator implementation
        loop {
            if let Some(ref mut inner) = self.front {
                if let Some(x) = inner.next() {
                    return Some(x);
                }
            }
            match self.pairs.next() {
                None => return self.back.as_mut().and_then(|it| it.next()),
                next => self.front = next.map(u4x2::iter_mut),
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<'a> DoubleEndedIterator for NibblesMut<'a> {
    fn next_back(&mut self) -> Option<&'a U4Cell> {
        // taken directly from core::iter::FlatMap source;
        // we can't use FlatMap because it doesn't have an ExactSizeIterator implementation
        loop {
            if let Some(ref mut inner) = self.back {
                if let Some(x) = inner.next_back() {
                    return Some(x);
                }
            }
            match self.pairs.next_back() {
                None => return self.front.as_mut().and_then(|it| it.next_back()),
                next => self.back = next.map(u4x2::iter_mut),
            }
        }
    }
}
impl<'a> ExactSizeIterator for NibblesMut<'a> {
    fn len(&self) -> usize {
        let front = self.front.as_ref().map(ExactSizeIterator::len).unwrap_or(0);
        let back = self.back.as_ref().map(ExactSizeIterator::len).unwrap_or(0);
        let middle = self.pairs.len() >> 1;
        front + middle + back
    }
}
