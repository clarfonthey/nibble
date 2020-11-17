//! Types for arrays of nibbles.
use core::ops;
use core::slice::{self as stdslice, from_raw_parts, from_raw_parts_mut};
use core::iter::FromIterator;
use arrayvec::{Array, ArrayVec, CapacityError};
use crate::base::{u4lo, u4};
use crate::pair::u4x2;
use crate::slice::{self, NibSliceAligned, NibSliceAlignedMut, NibSliceFull, NibSliceNoR};
use crate::common::{get_nib, shift_left, shift_right, set_nib};

/// An `ArrayVec` of nibbles.
#[derive(Clone)]
pub struct NibArrayVec<A: Array<Item = u4x2>>  {
    inner: ArrayVec<A>,
    has_right_lo: bool,
}
impl<A: Array<Item = u4x2>> NibArrayVec<A> {
    /// Creates an empty `NibArrayVec`.
    pub fn new() -> Self {
        NibArrayVec { inner: ArrayVec::new(), has_right_lo: true }
    }

    /// Number of nibbles in the vector.
    pub fn len(&self) -> usize {
        (self.inner.len() >> 1).saturating_sub(!self.has_right_lo as usize)
    }

    /// Whether the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// How many nibbles can be stored in the vector.
    pub fn capacity(&self) -> usize {
        self.inner.capacity() >> 1
    }

    /// Whether the vector is full.
    pub fn is_full(&self) -> bool {
        self.inner.is_full() && (self.inner.capacity() == 0 || self.has_right_lo)
    }

    /// Pushes a nibble onto the vector.
    ///
    /// # Panics
    ///
    /// Panics if the vector is full.
    pub fn push<T: u4>(&mut self, nib: T) {
        self.has_right_lo = !self.has_right_lo;
        if self.has_right_lo {
            self.inner.push(u4x2::from_hi(nib.to_u4hi()));
        } else {
            let i = self.inner.len() - 1;
            self.inner[i].set_lo(nib)
        }
    }

    /// Pushes a nibble onto the vector if possible.
    pub fn try_push<T: u4>(&mut self, nib: T) -> Result<(), CapacityError<T>> {
        if self.has_right_lo {
            match self.inner.try_push(u4x2::from_hi(nib.to_u4hi())) {
                Ok(()) => self.has_right_lo = false,
                Err(_) => return Err(CapacityError::new(nib)),
            }
        } else {
            let i = self.inner.len() - 1;
            self.inner[i].set_lo(nib);
        }
        Ok(())
    }

    /// Pushes a nibble onto the vector without checking if it's full.
    pub unsafe fn push_unchecked<T: u4>(&mut self, nib: T) {
        self.has_right_lo = !self.has_right_lo;
        if self.has_right_lo {
            self.inner.push_unchecked(u4x2::from_hi(nib.to_u4hi()));
        } else {
            let i = self.inner.len() - 1;
            self.inner[i].set_lo(nib)
        }
    }

    /// Inserts a nibble into the vector at the given index.
    pub fn insert<T: u4>(&mut self, index: usize, nib: T) {
        if self.has_right_lo {
            self.push(u4lo::from_lo(0));
        }
        shift_right(self.inner.as_mut_slice(), index);
        set_nib(self.inner.as_mut_slice(), index, nib);
    }

    /// Inserts a nibble into the vector at the given index.
    pub fn try_insert<T: u4>(&mut self, index: usize, nib: T) -> Result<(), CapacityError<T>> {
        let lo = nib.to_u4lo();
        if self.has_right_lo {
            self.inner.try_push(u4x2::from_byte(0)).map_err(|_| CapacityError::new(nib))?;
        }
        shift_right(self.inner.as_mut_slice(), index);
        set_nib(self.inner.as_mut_slice(), index, lo);
        Ok(())
    }

    fn discard_at(&mut self, index: usize) {
        shift_left(self.inner.as_mut_slice(), index);
        self.has_right_lo = !self.has_right_lo;
        if self.has_right_lo {
            self.inner.pop();
        }
    }

    /// Removes a nibble from the vector at the given index.
    pub fn remove<T: u4>(&mut self, index: usize) -> T {
        let ret = get_nib(self.inner.as_slice(), index);
        self.discard_at(index);
        ret
    }

    /// Removes a nibble from the vector at the given index, converting it to a high-order nibble.
    pub fn pop_at<T: u4>(&mut self, index: usize) -> Option<T> {
        if index >= self.len() {
            None
        } else {
            Some(self.remove(index))
        }
    }

    /// Removes a nibble from the vector, converting it to a high-order nibble.
    pub fn pop<T: u4>(&mut self) -> Option<T> {
        self.has_right_lo = !self.has_right_lo;
        if self.has_right_lo {
            Some(T::from_lo(self.inner[self.inner.len() - 1].lo().to_lo()))
        } else {
            self.inner.pop().map(|pair| T::from_hi(pair.hi().to_hi()))
        }
    }

    /// Clears the vector, removing all nibbles.
    pub fn clear(&mut self) {
        self.inner.clear();
        self.has_right_lo = true;
    }

    /// Converts the vector into an odd array, if it's full to one less than capacity.
    pub fn into_odd_array(self) -> Result<NibArrayOdd<A>, Self> {
        if self.inner.is_full() && !self.has_right_lo {
            Ok(NibArrayOdd { inner: self.inner.into_inner().unwrap() })
        } else {
            Err(self)
        }
    }

    /// Converts the vector into an even array, if it's full to capacity.
    pub fn into_even_array(self) -> Result<NibArrayEven<A>, Self> {
        if self.inner.is_full() && self.has_right_lo {
            Ok(NibArrayEven { inner: self.inner.into_inner().unwrap() })
        } else {
            Err(self)
        }
    }

    /// Intreprets this array as a slice.
    pub fn as_slice(&self) -> NibSliceAligned {
        if self.has_right_lo {
            NibSliceAligned::Even(unsafe { &*(&self.inner[..] as *const [u4x2] as *const NibSliceFull) })
        } else {
            NibSliceAligned::Odd(unsafe { &*(&self.inner[..] as *const [u4x2] as *const NibSliceNoR) })
        }
    }

    /// Intreprets this array as a mutable slice.
    pub fn as_mut_slice(&mut self) -> NibSliceAlignedMut {
        if self.has_right_lo {
            NibSliceAlignedMut::Even(unsafe { &mut *(&mut self.inner[..] as *mut [u4x2] as *mut NibSliceFull) })
        } else {
            NibSliceAlignedMut::Odd(unsafe { &mut *(&mut self.inner[..] as *mut [u4x2] as *mut NibSliceNoR) })
        }
    }
}
impl<A: Array<Item = u4x2>> Default for NibArrayVec<A> {
    fn default() -> Self {
        NibArrayVec::new()
    }
}
impl<A: Array<Item = u4x2>, T: u4> FromIterator<T> for NibArrayVec<A> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Self::new();
        vec.extend(iter);
        vec
    }
}
impl<'a, A: Array<Item = u4x2>> FromIterator<&'a dyn u4> for NibArrayVec<A> {
    fn from_iter<I: IntoIterator<Item = &'a dyn u4>>(iter: I) -> Self {
        let mut vec = Self::new();
        vec.extend(iter);
        vec
    }
}
impl<A: Array<Item = u4x2>> FromIterator<u4x2> for NibArrayVec<A> {
    fn from_iter<I: IntoIterator<Item = u4x2>>(iter: I) -> Self {
        let mut vec = Self::new();
        vec.extend(iter);
        vec
    }
}
impl<A: Array<Item = u4x2>, T: u4> Extend<T> for NibArrayVec<A> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for nib in iter {
            self.push(nib);
        }
    }
}
impl<'a, A: Array<Item = u4x2>> Extend<&'a dyn u4> for NibArrayVec<A> {
    fn extend<I: IntoIterator<Item = &'a dyn u4>>(&mut self, iter: I) {
        for nib in iter {
            self.push(nib.to_u4lo());
        }
    }
}
impl<A: Array<Item = u4x2>> Extend<u4x2> for NibArrayVec<A> {
    fn extend<I: IntoIterator<Item = u4x2>>(&mut self, iter: I) {
        for nib in iter {
            self.push(*nib.hi());
            self.push(*nib.lo());
        }
    }
}
impl<A: Array<Item = u4x2>> slice::private::Sealed for NibArrayVec<A> {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { true }
    #[inline(always)]
    fn has_right_lo(&self) -> bool { self.as_slice().has_right_lo() }
    #[inline(always)]
    fn iter(&self) -> stdslice::Iter<u4x2> { self.inner.iter() }
}
impl<A: Array<Item = u4x2>> slice::private::SealedMut for NibArrayVec<A> {
    #[inline(always)]
    fn iter_mut(&mut self) -> stdslice::IterMut<u4x2> { self.inner.iter_mut() }
}
impl<A: Array<Item = u4x2>> slice::NibSliceExt for NibArrayVec<A> {}
impl<A: Array<Item = u4x2>> slice::NibSliceMutExt for NibArrayVec<A> {}

/// An array with an even number of nibbles.
pub struct NibArrayEven<A: Array<Item = u4x2>> {
    inner: A,
}
impl<A: Array<Item = u4x2>> From<A> for NibArrayEven<A> {
    fn from(inner: A) -> Self {
        NibArrayEven { inner }
    }
}
impl<A: Array<Item = u4x2>> ops::Deref for NibArrayEven<A> {
    type Target = NibSliceFull;
    fn deref(&self) -> &NibSliceFull {
        let slice = unsafe { from_raw_parts(self.inner.as_slice().as_ptr(), A::CAPACITY) };
        unsafe { &*(slice as *const [u4x2] as *const NibSliceFull) }
    }
}
impl<A: Array<Item = u4x2>> ops::DerefMut for NibArrayEven<A> {
    fn deref_mut(&mut self) -> &mut NibSliceFull {
        let slice = unsafe { from_raw_parts_mut(self.inner.as_mut_slice().as_mut_ptr(), A::CAPACITY) };
        unsafe { &mut *(slice as *mut [u4x2] as *mut NibSliceFull) }
    }
}
impl<A: Array<Item = u4x2>> slice::private::Sealed for NibArrayEven<A> {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { true }
    #[inline(always)]
    fn has_right_lo(&self) -> bool { true }
    #[inline(always)]
    fn iter(&self) -> stdslice::Iter<u4x2> {
        unsafe { stdslice::from_raw_parts(self.inner.as_slice().as_ptr(), A::CAPACITY) }.iter()
    }
}
impl<A: Array<Item = u4x2>> slice::private::SealedMut for NibArrayEven<A> {
    #[inline(always)]
    fn iter_mut(&mut self) -> stdslice::IterMut<u4x2> {
        unsafe { stdslice::from_raw_parts_mut(self.inner.as_mut_slice().as_mut_ptr(), A::CAPACITY) }.iter_mut()
    }
}
impl<A: Array<Item = u4x2>> slice::NibSliceExt for NibArrayEven<A> {}
impl<A: Array<Item = u4x2>> slice::NibSliceMutExt for NibArrayEven<A> {}

/// An array with an odd number of nibbles.
pub struct NibArrayOdd<A: Array<Item = u4x2>> {
    inner: A
}
impl<A: Array<Item = u4x2>> From<A> for NibArrayOdd<A> {
    fn from(inner: A) -> Self {
        NibArrayOdd { inner }
    }
}
impl<A: Array<Item = u4x2>> ops::Deref for NibArrayOdd<A> {
    type Target = NibSliceNoR;
    fn deref(&self) -> &NibSliceNoR {
        let slice = unsafe { from_raw_parts(self.inner.as_slice().as_ptr(), A::CAPACITY) };
        unsafe { &*(slice as *const [u4x2] as *const NibSliceNoR) }
    }
}
impl<A: Array<Item = u4x2>> ops::DerefMut for NibArrayOdd<A> {
    fn deref_mut(&mut self) -> &mut NibSliceNoR {
        let slice = unsafe { from_raw_parts_mut(self.inner.as_mut_slice().as_mut_ptr(), A::CAPACITY) };
        unsafe { &mut *(slice as *mut [u4x2] as *mut NibSliceNoR) }
    }
}
impl<A: Array<Item = u4x2>> slice::private::Sealed for NibArrayOdd<A> {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { true }
    #[inline(always)]
    fn has_right_lo(&self) -> bool { false }
    #[inline(always)]
    fn iter(&self) -> stdslice::Iter<u4x2> {
        unsafe { stdslice::from_raw_parts(self.inner.as_slice().as_ptr(), A::CAPACITY) }.iter()
    }
}
impl<A: Array<Item = u4x2>> slice::private::SealedMut for NibArrayOdd<A> {
    #[inline(always)]
    fn iter_mut(&mut self) -> stdslice::IterMut<u4x2> {
        unsafe { stdslice::from_raw_parts_mut(self.inner.as_mut_slice().as_mut_ptr(), A::CAPACITY) }.iter_mut()
    }
}
impl<A: Array<Item = u4x2>> slice::NibSliceExt for NibArrayOdd<A> {}
impl<A: Array<Item = u4x2>> slice::NibSliceMutExt for NibArrayOdd<A> {}

/// An array of nibbles.
pub enum NibArray<A: Array<Item = u4x2>> {
    Even(NibArrayEven<A>),
    Odd(NibArrayOdd<A>),
}
impl<A: Array<Item = u4x2>> NibArray<A> {
    /// Intreprets this array as a slice.
    pub fn as_slice(&self) -> NibSliceAligned {
        match *self {
            NibArray::Even(ref e) => NibSliceAligned::Even(e),
            NibArray::Odd(ref e) => NibSliceAligned::Odd(e),
        }
    }

    /// Intreprets this array as a mutable slice.
    pub fn as_mut_slice(&mut self) -> NibSliceAlignedMut {
        match *self {
            NibArray::Even(ref mut e) => NibSliceAlignedMut::Even(e),
            NibArray::Odd(ref mut e) => NibSliceAlignedMut::Odd(e),
        }
    }
}
impl<A: Array<Item = u4x2>> slice::private::Sealed for NibArray<A> {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { true }

    fn has_right_lo(&self) -> bool {
        match *self {
            NibArray::Even(_) => true,
            NibArray::Odd(_) => false,
        }
    }

    fn iter(&self) -> stdslice::Iter<u4x2> {
        match *self {
            NibArray::Even(ref s) => unsafe { stdslice::from_raw_parts(s.inner.as_slice().as_ptr(), A::CAPACITY) }.iter(),
            NibArray::Odd(ref s) => unsafe { stdslice::from_raw_parts(s.inner.as_slice().as_ptr(), A::CAPACITY) }.iter(),
        }
    }
}
impl<A: Array<Item = u4x2>> slice::private::SealedMut for NibArray<A> {
    fn iter_mut(&mut self) -> stdslice::IterMut<u4x2> {
        match *self {
            NibArray::Even(ref mut s) => unsafe { stdslice::from_raw_parts_mut(s.inner.as_mut_slice().as_mut_ptr(), A::CAPACITY) }.iter_mut(),
            NibArray::Odd(ref mut s) => unsafe { stdslice::from_raw_parts_mut(s.inner.as_mut_slice().as_mut_ptr(), A::CAPACITY) }.iter_mut(),
        }
    }
}
impl<A: Array<Item = u4x2>> slice::NibSliceExt for NibArray<A> {}
impl<A: Array<Item = u4x2>> slice::NibSliceMutExt for NibArray<A> {}
