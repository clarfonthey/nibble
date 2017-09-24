//! Traits for dealing with slices of nibbles.
use core::slice;
use base::{u4hi, u4lo, u4};
use iter::{NibblePairs, NibblePairsMut, Nibbles, NibblesMut};
use pair::{U4HiCell, U4LoCell, U4Cell, u4x2};
use common::{get_nib_ref, get_nib_mut};

pub(crate) mod private {
    use super::{slice, u4x2};

    pub trait Sealed {
        fn has_left_hi(&self) -> bool;
        fn has_right_lo(&self) -> bool;
        fn iter(&self) -> slice::Iter<u4x2>;
    }
    pub trait SealedMut {
        fn iter_mut(&mut self) -> slice::IterMut<u4x2>;
    }
}

/// A slice of nibbles.
pub trait NibSliceExt: private::Sealed {
    /// Iterator over the nibble pairs in this slice.
    ///
    /// This may include nibbles that are omitted.
    fn nibble_pairs(&self) -> NibblePairs {
        NibblePairs::new(self.iter())
    }

    /// Iterator over nibbles in a slice.
    fn nibbles(&self) -> Nibbles {
        let has_left_hi = self.has_left_hi();
        let has_right_lo = self.has_right_lo();
        Nibbles::new(self.nibble_pairs(), has_left_hi, has_right_lo)
    }

    /// Decomposes this slice into its parts.
    fn decompose(&self) -> (Option<&u4lo>, &[u4x2], Option<&u4hi>) {
        let has_left_hi = self.has_left_hi();
        let has_right_lo = self.has_right_lo();
        let slice = self.nibble_pairs().as_slice();
        match slice.len() {
            0 => (None, slice, None),
            1 if !has_left_hi && !has_right_lo => (None, &slice[1..], None),
            _ => {
                let (left, slice) = if has_left_hi {
                    (None, slice)
                } else {
                    slice.split_first()
                        .map(|(p, s)| (Some(p.lo()), s))
                        .unwrap_or_else(|| unreachable!())
                };
                let (right, slice) = if has_right_lo {
                    (None, slice)
                } else {
                    slice.split_last()
                        .map(|(p, s)| (Some(p.hi()), s))
                        .unwrap_or_else(|| unreachable!())
                };
                (left, slice, right)
            }
        }
    }

    /// Gets a nibble at the given index.
    fn get(&self, idx: usize) -> &u4 {
        get_nib_ref(self.iter().as_slice(), idx)
    }

    /// Gets the length of the slice.
    fn len(&self) -> usize {
        let hi = self.has_left_hi() as usize;
        let lo = self.has_right_lo() as usize;
        self.iter().as_slice().len().saturating_sub(hi + lo)
    }

    /// Checks if the slice is empty.
    fn is_empty(&self) -> bool {
        self.iter().as_slice().is_empty()
    }

    /// Converts this slice into a `NibSlice`.
    fn into_generic(&self) -> NibSlice {
        if self.has_left_hi() {
            if self.has_right_lo() {
                NibSliceFull::from_slice(self.iter().as_slice()).into()
            } else {
                NibSliceNoR::from_slice(self.iter().as_slice()).into()
            }
        } else {
            if self.has_right_lo() {
                NibSliceNoL::from_slice(self.iter().as_slice()).into()
            } else {
                NibSliceNoBoth::from_slice(self.iter().as_slice()).into()
            }
        }
    }

    /// Checks whether this slice is aligned to a byte boundary.
    fn is_aligned(&self) -> bool {
        self.has_left_hi()
    }

    /// Checks whether this slice has an even number of nibbles.
    fn is_even(&self) -> bool {
        self.has_left_hi() == self.has_right_lo()
    }

    /// Checks whether this slice has an odd number of nibbles.
    fn is_odd(&self) -> bool {
        self.has_left_hi() != self.has_right_lo()
    }
}

/// A mutable slice of nibbles.
pub trait NibSliceMutExt: NibSliceExt + private::SealedMut {
    /// Mutable iterator over the nibble pairs in this slice.
    ///
    /// This may include nibbles that are omitted.
    fn nibble_pairs_mut(&mut self) -> NibblePairsMut {
        NibblePairsMut::new(self.iter_mut())
    }

    /// Mutable iterator over nibbles in a slice.
    fn nibbles_mut(&mut self) -> NibblesMut {
        let has_left_hi = self.has_left_hi();
        let has_right_lo = self.has_right_lo();
        NibblesMut::new(self.nibble_pairs_mut(), has_left_hi, has_right_lo)
    }

    /// Mutably decomposes this slice into its parts.
    fn decompose_mut(&mut self) -> (Option<&U4LoCell>, &mut [u4x2], Option<&U4HiCell>) {
        let has_left_hi = self.has_left_hi();
        let has_right_lo = self.has_right_lo();
        let slice = self.nibble_pairs_mut().into_slice();
        match slice.len() {
            0 => (None, slice, None),
            1 if !has_left_hi && !has_right_lo => (None, &mut slice[1..], None),
            _ => {
                let (left, slice) = if has_left_hi {
                    (None, slice)
                } else {
                    slice.split_first_mut()
                        .map(|(p, s)| (Some(p.lo_mut()), s))
                        .unwrap_or_else(|| unreachable!())
                };
                let (right, slice) = if has_right_lo {
                    (None, slice)
                } else {
                    slice.split_last_mut()
                        .map(|(p, s)| (Some(p.hi_mut()), s))
                        .unwrap_or_else(|| unreachable!())
                };
                (left, slice, right)
            }
        }
    }

    /// Mutably gets a nibble at the given index.
    fn get_mut(&mut self, idx: usize) -> &U4Cell {
        get_nib_mut(self.iter_mut().into_slice(), idx)
    }

    /// Converts this slice into a `NibSliceMut`.
    fn into_generic_mut(&mut self) -> NibSliceMut {
        if self.has_left_hi() {
            if self.has_right_lo() {
                NibSliceFull::from_mut_slice(self.iter_mut().into_slice()).into()
            } else {
                NibSliceNoR::from_mut_slice(self.iter_mut().into_slice()).into()
            }
        } else {
            if self.has_right_lo() {
                NibSliceNoL::from_mut_slice(self.iter_mut().into_slice()).into()
            } else {
                NibSliceNoBoth::from_mut_slice(self.iter_mut().into_slice()).into()
            }
        }
    }
}

/// Nibble slice which only contains complete pairs.
pub struct NibSliceFull {
    inner: [u4x2],
}
impl NibSliceFull {
    pub(crate) fn from_slice(slice: &[u4x2]) -> &Self {
        unsafe { &*(slice as *const [u4x2] as *const Self) }
    }
    pub(crate) fn from_mut_slice(slice: &mut [u4x2]) -> &mut Self {
        unsafe { &mut *(slice as *mut [u4x2] as *mut Self) }
    }
}
impl private::Sealed for NibSliceFull {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { true }
    #[inline(always)]
    fn has_right_lo(&self) -> bool { true }
    #[inline(always)]
    fn iter(&self) -> slice::Iter<u4x2> { self.inner.iter() }
}
impl private::SealedMut for NibSliceFull {
    #[inline(always)]
    fn iter_mut(&mut self) -> slice::IterMut<u4x2> { self.inner.iter_mut() }
}
impl NibSliceExt for NibSliceFull {}
impl NibSliceMutExt for NibSliceFull {}

/// Nibble slice which is missing the rightmost, low-order nibble.
pub struct NibSliceNoR {
    inner: [u4x2],
}
impl NibSliceNoR {
    pub(crate) fn from_slice(slice: &[u4x2]) -> &Self {
        unsafe { &*(slice as *const [u4x2] as *const Self) }
    }
    pub(crate) fn from_mut_slice(slice: &mut [u4x2]) -> &mut Self {
        unsafe { &mut *(slice as *mut [u4x2] as *mut Self) }
    }
}
impl private::Sealed for NibSliceNoR {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { true }
    #[inline(always)]
    fn has_right_lo(&self) -> bool { false }
    #[inline(always)]
    fn iter(&self) -> slice::Iter<u4x2> { self.inner.iter() }
}
impl private::SealedMut for NibSliceNoR {
    #[inline(always)]
    fn iter_mut(&mut self) -> slice::IterMut<u4x2> { self.inner.iter_mut() }
}
impl NibSliceExt for NibSliceNoR {}
impl NibSliceMutExt for NibSliceNoR {}

/// Nibble slice which is missing the leftmost, high-order nibble.
pub struct NibSliceNoL {
    inner: [u4x2],
}
impl NibSliceNoL {
    pub(crate) fn from_slice(slice: &[u4x2]) -> &Self {
        unsafe { &*(slice as *const [u4x2] as *const Self) }
    }
    pub(crate) fn from_mut_slice(slice: &mut [u4x2]) -> &mut Self {
        unsafe { &mut *(slice as *mut [u4x2] as *mut Self) }
    }
}
impl private::Sealed for NibSliceNoL {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { false }
    #[inline(always)]
    fn has_right_lo(&self) -> bool { true }
    #[inline(always)]
    fn iter(&self) -> slice::Iter<u4x2> { self.inner.iter() }
}
impl private::SealedMut for NibSliceNoL {
    #[inline(always)]
    fn iter_mut(&mut self) -> slice::IterMut<u4x2> { self.inner.iter_mut() }
}
impl NibSliceExt for NibSliceNoL {}
impl NibSliceMutExt for NibSliceNoL {}

/// Nibble slice which is missing the leftmost, high-order and the rightmost, low-order nibbles.
pub struct NibSliceNoBoth {
    inner: [u4x2],
}
impl NibSliceNoBoth {
    pub(crate) fn from_slice(slice: &[u4x2]) -> &Self {
        unsafe { &*(slice as *const [u4x2] as *const Self) }
    }
    pub(crate) fn from_mut_slice(slice: &mut [u4x2]) -> &mut Self {
        unsafe { &mut *(slice as *mut [u4x2] as *mut Self) }
    }
}
impl private::Sealed for NibSliceNoBoth {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { false }
    #[inline(always)]
    fn has_right_lo(&self) -> bool { false }
    #[inline(always)]
    fn iter(&self) -> slice::Iter<u4x2> { self.inner.iter() }
}
impl private::SealedMut for NibSliceNoBoth {
    #[inline(always)]
    fn iter_mut(&mut self) -> slice::IterMut<u4x2> { self.inner.iter_mut() }
}
impl NibSliceExt for NibSliceNoBoth {}
impl NibSliceMutExt for NibSliceNoBoth {}

/// Reference to a nibble slice that's aligned to a byte boundary.
pub enum NibSliceAligned<'a> {
    /// An aligned slice with an even number of nibbles.
    Even(&'a NibSliceFull),
    /// An aligned slice with an odd number of nibbles.
    Odd(&'a NibSliceNoR),
}
impl<'a> From<&'a NibSliceFull> for NibSliceAligned<'a> {
    fn from(slice: &'a NibSliceFull) -> Self {
        NibSliceAligned::Even(slice)
    }
}
impl<'a> From<&'a NibSliceNoR> for NibSliceAligned<'a> {
    fn from(slice: &'a NibSliceNoR) -> Self {
        NibSliceAligned::Odd(slice)
    }
}
impl<'a> private::Sealed for NibSliceAligned<'a> {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { true }
    #[inline]
    fn has_right_lo(&self) -> bool {
        match *self {
            NibSliceAligned::Even(_) => true,
            NibSliceAligned::Odd(_) => false,
        }
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSliceAligned::Even(ref s) => s.iter(),
            NibSliceAligned::Odd(ref s) => s.iter(),
        }
    }
}
impl<'a> NibSliceExt for NibSliceAligned<'a> {}

/// Mutable reference to a nibble slice that's aligned to a byte boundary.
pub enum NibSliceAlignedMut<'a> {
    /// An aligned slice with an even number of nibbles.
    Even(&'a mut NibSliceFull),
    /// An aligned slice with an odd number of nibbles.
    Odd(&'a mut NibSliceNoR),
}
impl<'a> From<&'a mut NibSliceFull> for NibSliceAlignedMut<'a> {
    fn from(slice: &'a mut NibSliceFull) -> Self {
        NibSliceAlignedMut::Even(slice)
    }
}
impl<'a> From<&'a mut NibSliceNoR> for NibSliceAlignedMut<'a> {
    fn from(slice: &'a mut NibSliceNoR) -> Self {
        NibSliceAlignedMut::Odd(slice)
    }
}
impl<'a> private::Sealed for NibSliceAlignedMut<'a> {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { true }
    #[inline]
    fn has_right_lo(&self) -> bool {
        match *self {
            NibSliceAlignedMut::Even(_) => true,
            NibSliceAlignedMut::Odd(_) => false,
        }
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSliceAlignedMut::Even(ref s) => s.iter(),
            NibSliceAlignedMut::Odd(ref s) => s.iter(),
        }
    }
}
impl<'a> private::SealedMut for NibSliceAlignedMut<'a> {
    #[inline]
    fn iter_mut(&mut self) -> slice::IterMut<u4x2> {
        match *self {
            NibSliceAlignedMut::Even(ref mut s) => s.iter_mut(),
            NibSliceAlignedMut::Odd(ref mut s) => s.iter_mut(),
        }
    }
}
impl<'a> NibSliceExt for NibSliceAlignedMut<'a> {}
impl<'a> NibSliceMutExt for NibSliceAlignedMut<'a> {}

/// Reference to a nibble slice that's not aligned to a byte boundary.
pub enum NibSliceUnaligned<'a> {
    /// An unaligned slice with an even number of nibbles.
    Even(&'a NibSliceNoBoth),
    /// An unaligned slice with an odd number of nibbles.
    Odd(&'a NibSliceNoL),
}
impl<'a> From<&'a NibSliceNoBoth> for NibSliceUnaligned<'a> {
    fn from(slice: &'a NibSliceNoBoth) -> Self {
        NibSliceUnaligned::Even(slice)
    }
}
impl<'a> From<&'a NibSliceNoL> for NibSliceUnaligned<'a> {
    fn from(slice: &'a NibSliceNoL) -> Self {
        NibSliceUnaligned::Odd(slice)
    }
}
impl<'a> private::Sealed for NibSliceUnaligned<'a> {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { false }
    #[inline]
    fn has_right_lo(&self) -> bool {
        match *self {
            NibSliceUnaligned::Even(_) => false,
            NibSliceUnaligned::Odd(_) => true,
        }
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSliceUnaligned::Even(ref s) => s.iter(),
            NibSliceUnaligned::Odd(ref s) => s.iter(),
        }
    }
}
impl<'a> NibSliceExt for NibSliceUnaligned<'a> {}

/// Mutable reference to a nibble slice that's not aligned to a byte boundary.
pub enum NibSliceUnalignedMut<'a> {
    /// An unaligned slice with an even number of nibbles.
    Even(&'a mut NibSliceNoBoth),
    /// An unaligned slice with an odd number of nibbles.
    Odd(&'a mut NibSliceNoL),
}
impl<'a> From<&'a mut NibSliceNoBoth> for NibSliceUnalignedMut<'a> {
    fn from(slice: &'a mut NibSliceNoBoth) -> Self {
        NibSliceUnalignedMut::Even(slice)
    }
}
impl<'a> From<&'a mut NibSliceNoL> for NibSliceUnalignedMut<'a> {
    fn from(slice: &'a mut NibSliceNoL) -> Self {
        NibSliceUnalignedMut::Odd(slice)
    }
}
impl<'a> private::Sealed for NibSliceUnalignedMut<'a> {
    #[inline(always)]
    fn has_left_hi(&self) -> bool { false }
    #[inline]
    fn has_right_lo(&self) -> bool {
        match *self {
            NibSliceUnalignedMut::Even(_) => false,
            NibSliceUnalignedMut::Odd(_) => true,
        }
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSliceUnalignedMut::Even(ref s) => s.iter(),
            NibSliceUnalignedMut::Odd(ref s) => s.iter(),
        }
    }
}
impl<'a> private::SealedMut for NibSliceUnalignedMut<'a> {
    #[inline]
    fn iter_mut(&mut self) -> slice::IterMut<u4x2> {
        match *self {
            NibSliceUnalignedMut::Even(ref mut s) => s.iter_mut(),
            NibSliceUnalignedMut::Odd(ref mut s) => s.iter_mut(),
        }
    }
}
impl<'a> NibSliceExt for NibSliceUnalignedMut<'a> {}
impl<'a> NibSliceMutExt for NibSliceUnalignedMut<'a> {}

/// Reference to a nibble slice with an even number of nibbles.
pub enum NibSliceEven<'a> {
    /// An aligned slice with an even number of nibbles.
    Aligned(&'a NibSliceFull),
    /// An unaligned slice with an even number of nibbles.
    Unaligned(&'a NibSliceNoBoth),
}
impl<'a> From<&'a NibSliceFull> for NibSliceEven<'a> {
    fn from(slice: &'a NibSliceFull) -> Self {
        NibSliceEven::Aligned(slice)
    }
}
impl<'a> From<&'a NibSliceNoBoth> for NibSliceEven<'a> {
    fn from(slice: &'a NibSliceNoBoth) -> Self {
        NibSliceEven::Unaligned(slice)
    }
}
impl<'a> private::Sealed for NibSliceEven<'a> {
    #[inline]
    fn has_left_hi(&self) -> bool {
        match *self {
            NibSliceEven::Aligned(_) => true,
            NibSliceEven::Unaligned(_) => false,
        }
    }
    #[inline]
    fn has_right_lo(&self) -> bool {
        self.has_left_hi()
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSliceEven::Aligned(ref s) => s.iter(),
            NibSliceEven::Unaligned(ref s) => s.iter(),
        }
    }
}
impl<'a> NibSliceExt for NibSliceEven<'a> {}

/// Mutable reference to a nibble slice with an even number of nibbles.
pub enum NibSliceEvenMut<'a> {
    /// An aligned slice with an even number of nibbles.
    Aligned(&'a mut NibSliceFull),
    /// An unaligned slice with an even number of nibbles.
    Unaligned(&'a mut NibSliceNoBoth),
}
impl<'a> From<&'a mut NibSliceFull> for NibSliceEvenMut<'a> {
    fn from(slice: &'a mut NibSliceFull) -> Self {
        NibSliceEvenMut::Aligned(slice)
    }
}
impl<'a> From<&'a mut NibSliceNoBoth> for NibSliceEvenMut<'a> {
    fn from(slice: &'a mut NibSliceNoBoth) -> Self {
        NibSliceEvenMut::Unaligned(slice)
    }
}
impl<'a> private::Sealed for NibSliceEvenMut<'a> {
    #[inline]
    fn has_left_hi(&self) -> bool {
        match *self {
            NibSliceEvenMut::Aligned(_) => true,
            NibSliceEvenMut::Unaligned(_) => false,
        }
    }
    #[inline]
    fn has_right_lo(&self) -> bool {
        self.has_left_hi()
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSliceEvenMut::Aligned(ref s) => s.iter(),
            NibSliceEvenMut::Unaligned(ref s) => s.iter(),
        }
    }
}
impl<'a> private::SealedMut for NibSliceEvenMut<'a> {
    #[inline]
    fn iter_mut(&mut self) -> slice::IterMut<u4x2> {
        match *self {
            NibSliceEvenMut::Aligned(ref mut s) => s.iter_mut(),
            NibSliceEvenMut::Unaligned(ref mut s) => s.iter_mut(),
        }
    }
}
impl<'a> NibSliceExt for NibSliceEvenMut<'a> {}
impl<'a> NibSliceMutExt for NibSliceEvenMut<'a> {}

/// Reference to a nibble slice with an odd number of nibbles.
pub enum NibSliceOdd<'a> {
    /// An aligned slice with an odd number of nibbles.
    Aligned(&'a NibSliceNoR),
    /// An unaligned slice with an odd number of nibbles.
    Unaligned(&'a NibSliceNoL),
}
impl<'a> From<&'a NibSliceNoR> for NibSliceOdd<'a> {
    fn from(slice: &'a NibSliceNoR) -> Self {
        NibSliceOdd::Aligned(slice)
    }
}
impl<'a> From<&'a NibSliceNoL> for NibSliceOdd<'a> {
    fn from(slice: &'a NibSliceNoL) -> Self {
        NibSliceOdd::Unaligned(slice)
    }
}
impl<'a> private::Sealed for NibSliceOdd<'a> {
    #[inline]
    fn has_left_hi(&self) -> bool {
        match *self {
            NibSliceOdd::Aligned(_) => true,
            NibSliceOdd::Unaligned(_) => false,
        }
    }
    #[inline]
    fn has_right_lo(&self) -> bool {
        !self.has_left_hi()
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSliceOdd::Aligned(ref s) => s.iter(),
            NibSliceOdd::Unaligned(ref s) => s.iter(),
        }
    }
}
impl<'a> NibSliceExt for NibSliceOdd<'a> {}

/// Reference to a nibble slice with an odd number of nibbles.
pub enum NibSliceOddMut<'a> {
    /// An aligned slice with an odd number of nibbles.
    Aligned(&'a mut NibSliceNoR),
    /// An unaligned slice with an odd number of nibbles.
    Unaligned(&'a mut NibSliceNoL),
}
impl<'a> From<&'a mut NibSliceNoR> for NibSliceOddMut<'a> {
    fn from(slice: &'a mut NibSliceNoR) -> Self {
        NibSliceOddMut::Aligned(slice)
    }
}
impl<'a> From<&'a mut NibSliceNoL> for NibSliceOddMut<'a> {
    fn from(slice: &'a mut NibSliceNoL) -> Self {
        NibSliceOddMut::Unaligned(slice)
    }
}
impl<'a> private::Sealed for NibSliceOddMut<'a> {
    #[inline]
    fn has_left_hi(&self) -> bool {
        match *self {
            NibSliceOddMut::Aligned(_) => true,
            NibSliceOddMut::Unaligned(_) => false,
        }
    }
    #[inline]
    fn has_right_lo(&self) -> bool {
        !self.has_left_hi()
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSliceOddMut::Aligned(ref s) => s.iter(),
            NibSliceOddMut::Unaligned(ref s) => s.iter(),
        }
    }
}
impl<'a> private::SealedMut for NibSliceOddMut<'a> {
    #[inline]
    fn iter_mut(&mut self) -> slice::IterMut<u4x2> {
        match *self {
            NibSliceOddMut::Aligned(ref mut s) => s.iter_mut(),
            NibSliceOddMut::Unaligned(ref mut s) => s.iter_mut(),
        }
    }
}
impl<'a> NibSliceExt for NibSliceOddMut<'a> {}
impl<'a> NibSliceMutExt for NibSliceOddMut<'a> {}

/// Reference to a nibble slice.
pub enum NibSlice<'a> {
    /// A slice with both sides.
    Full(&'a NibSliceFull),

    /// A slice without the left side.
    NoL(&'a NibSliceNoL),

    /// A slice without the right side.
    NoR(&'a NibSliceNoR),

    /// A slice without both sides.
    NoBoth(&'a NibSliceNoBoth),
}
impl<'a> From<&'a NibSliceFull> for NibSlice<'a> {
    fn from(slice: &'a NibSliceFull) -> Self {
        NibSlice::Full(slice)
    }
}
impl<'a> From<&'a NibSliceNoL> for NibSlice<'a> {
    fn from(slice: &'a NibSliceNoL) -> Self {
        NibSlice::NoL(slice)
    }
}
impl<'a> From<&'a NibSliceNoR> for NibSlice<'a> {
    fn from(slice: &'a NibSliceNoR) -> Self {
        NibSlice::NoR(slice)
    }
}
impl<'a> From<&'a NibSliceNoBoth> for NibSlice<'a> {
    fn from(slice: &'a NibSliceNoBoth) -> Self {
        NibSlice::NoBoth(slice)
    }
}
impl<'a> From<NibSliceAligned<'a>> for NibSlice<'a> {
    fn from(slice: NibSliceAligned<'a>) -> Self {
        match slice {
            NibSliceAligned::Even(s) => NibSlice::Full(s),
            NibSliceAligned::Odd(s) => NibSlice::NoR(s),
        }
    }
}
impl<'a> From<NibSliceUnaligned<'a>> for NibSlice<'a> {
    fn from(slice: NibSliceUnaligned<'a>) -> Self {
        match slice {
            NibSliceUnaligned::Even(s) => NibSlice::NoBoth(s),
            NibSliceUnaligned::Odd(s) => NibSlice::NoL(s),
        }
    }
}
impl<'a> From<NibSliceEven<'a>> for NibSlice<'a> {
    fn from(slice: NibSliceEven<'a>) -> Self {
        match slice {
            NibSliceEven::Aligned(s) => NibSlice::Full(s),
            NibSliceEven::Unaligned(s) => NibSlice::NoBoth(s),
        }
    }
}
impl<'a> From<NibSliceOdd<'a>> for NibSlice<'a> {
    fn from(slice: NibSliceOdd<'a>) -> Self {
        match slice {
            NibSliceOdd::Aligned(s) => NibSlice::NoR(s),
            NibSliceOdd::Unaligned(s) => NibSlice::NoL(s),
        }
    }
}
impl<'a> NibSlice<'a> {
    /// Converts this slice into an aligned version.
    pub fn as_aligned(&self) -> Option<NibSliceAligned<'a>> {
        match *self {
            NibSlice::Full(s) => Some(s.into()),
            NibSlice::NoL(_) => None,
            NibSlice::NoR(s) => Some(s.into()),
            NibSlice::NoBoth(_) => None,
        }
    }
    /// Converts this slice into an unaligned version.
    pub fn as_unaligned(&self) -> Option<NibSliceUnaligned<'a>> {
        match *self {
            NibSlice::Full(_) => None,
            NibSlice::NoL(s) => Some(s.into()),
            NibSlice::NoR(_) => None,
            NibSlice::NoBoth(s) => Some(s.into()),
        }
    }
    /// Converts this slice into an even version.
    pub fn as_even(&self) -> Option<NibSliceEven<'a>> {
        match *self {
            NibSlice::Full(s) => Some(s.into()),
            NibSlice::NoL(_) => None,
            NibSlice::NoR(_) => None,
            NibSlice::NoBoth(s) => Some(s.into()),
        }
    }
    /// Converts this slice into an odd version.
    pub fn as_odd(&self) -> Option<NibSliceOdd<'a>> {
        match *self {
            NibSlice::Full(_) => None,
            NibSlice::NoL(s) => Some(s.into()),
            NibSlice::NoR(s) => Some(s.into()),
            NibSlice::NoBoth(_) => None,
        }
    }
}
impl<'a> private::Sealed for NibSlice<'a> {
    #[inline]
    fn has_left_hi(&self) -> bool {
        match *self {
            NibSlice::Full(_) => true,
            NibSlice::NoL(_) => false,
            NibSlice::NoR(_) => true,
            NibSlice::NoBoth(_) => false,
        }
    }
    #[inline]
    fn has_right_lo(&self) -> bool {
        match *self {
            NibSlice::Full(_) => true,
            NibSlice::NoL(_) => true,
            NibSlice::NoR(_) => false,
            NibSlice::NoBoth(_) => false,
        }
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSlice::Full(ref s) => s.iter(),
            NibSlice::NoL(ref s) => s.iter(),
            NibSlice::NoR(ref s) => s.iter(),
            NibSlice::NoBoth(ref s) => s.iter(),
        }
    }
}
impl<'a> NibSliceExt for NibSlice<'a> {}

/// Mutable reference to a nibble slice.
pub enum NibSliceMut<'a> {
    /// A slice with both sides.
    Full(&'a mut NibSliceFull),

    /// A slice without the left side.
    NoL(&'a mut NibSliceNoL),

    /// A slice without the right side.
    NoR(&'a mut NibSliceNoR),

    /// A slice without both sides.
    NoBoth(&'a mut NibSliceNoBoth),
}
impl<'a> From<&'a mut NibSliceFull> for NibSliceMut<'a> {
    fn from(slice: &'a mut NibSliceFull) -> Self {
        NibSliceMut::Full(slice)
    }
}
impl<'a> From<&'a mut NibSliceNoL> for NibSliceMut<'a> {
    fn from(slice: &'a mut NibSliceNoL) -> Self {
        NibSliceMut::NoL(slice)
    }
}
impl<'a> From<&'a mut NibSliceNoR> for NibSliceMut<'a> {
    fn from(slice: &'a mut NibSliceNoR) -> Self {
        NibSliceMut::NoR(slice)
    }
}
impl<'a> From<&'a mut NibSliceNoBoth> for NibSliceMut<'a> {
    fn from(slice: &'a mut NibSliceNoBoth) -> Self {
        NibSliceMut::NoBoth(slice)
    }
}
impl<'a> From<NibSliceAlignedMut<'a>> for NibSliceMut<'a> {
    fn from(slice: NibSliceAlignedMut<'a>) -> Self {
        match slice {
            NibSliceAlignedMut::Even(s) => NibSliceMut::Full(s),
            NibSliceAlignedMut::Odd(s) => NibSliceMut::NoR(s),
        }
    }
}
impl<'a> From<NibSliceUnalignedMut<'a>> for NibSliceMut<'a> {
    fn from(slice: NibSliceUnalignedMut<'a>) -> Self {
        match slice {
            NibSliceUnalignedMut::Even(s) => NibSliceMut::NoBoth(s),
            NibSliceUnalignedMut::Odd(s) => NibSliceMut::NoL(s),
        }
    }
}
impl<'a> From<NibSliceEvenMut<'a>> for NibSliceMut<'a> {
    fn from(slice: NibSliceEvenMut<'a>) -> Self {
        match slice {
            NibSliceEvenMut::Aligned(s) => NibSliceMut::Full(s),
            NibSliceEvenMut::Unaligned(s) => NibSliceMut::NoBoth(s),
        }
    }
}
impl<'a> From<NibSliceOddMut<'a>> for NibSliceMut<'a> {
    fn from(slice: NibSliceOddMut<'a>) -> Self {
        match slice {
            NibSliceOddMut::Aligned(s) => NibSliceMut::NoR(s),
            NibSliceOddMut::Unaligned(s) => NibSliceMut::NoL(s),
        }
    }
}
impl<'a> NibSliceMut<'a> {
    /// Converts this slice into an aligned version.
    pub fn into_aligned(self) -> Option<NibSliceAlignedMut<'a>> {
        match self {
            NibSliceMut::Full(s) => Some(s.into()),
            NibSliceMut::NoL(_) => None,
            NibSliceMut::NoR(s) => Some(s.into()),
            NibSliceMut::NoBoth(_) => None,
        }
    }
    /// Converts this slice into an unaligned version.
    pub fn into_unaligned(self) -> Option<NibSliceUnalignedMut<'a>> {
        match self {
            NibSliceMut::Full(_) => None,
            NibSliceMut::NoL(s) => Some(s.into()),
            NibSliceMut::NoR(_) => None,
            NibSliceMut::NoBoth(s) => Some(s.into()),
        }
    }
    /// Converts this slice into an even version.
    pub fn into_even(self) -> Option<NibSliceEvenMut<'a>> {
        match self {
            NibSliceMut::Full(s) => Some(s.into()),
            NibSliceMut::NoL(_) => None,
            NibSliceMut::NoR(_) => None,
            NibSliceMut::NoBoth(s) => Some(s.into()),
        }
    }
    /// Converts this slice into an odd version.
    pub fn into_odd(self) -> Option<NibSliceOddMut<'a>> {
        match self {
            NibSliceMut::Full(_) => None,
            NibSliceMut::NoL(s) => Some(s.into()),
            NibSliceMut::NoR(s) => Some(s.into()),
            NibSliceMut::NoBoth(_) => None,
        }
    }
}
impl<'a> private::Sealed for NibSliceMut<'a> {
    #[inline]
    fn has_left_hi(&self) -> bool {
        match *self {
            NibSliceMut::Full(_) => true,
            NibSliceMut::NoL(_) => false,
            NibSliceMut::NoR(_) => true,
            NibSliceMut::NoBoth(_) => false,
        }
    }
    #[inline]
    fn has_right_lo(&self) -> bool {
        match *self {
            NibSliceMut::Full(_) => true,
            NibSliceMut::NoL(_) => true,
            NibSliceMut::NoR(_) => false,
            NibSliceMut::NoBoth(_) => false,
        }
    }
    #[inline]
    fn iter(&self) -> slice::Iter<u4x2> {
        match *self {
            NibSliceMut::Full(ref s) => s.iter(),
            NibSliceMut::NoL(ref s) => s.iter(),
            NibSliceMut::NoR(ref s) => s.iter(),
            NibSliceMut::NoBoth(ref s) => s.iter(),
        }
    }
}
impl<'a> private::SealedMut for NibSliceMut<'a> {
    #[inline]
    fn iter_mut(&mut self) -> slice::IterMut<u4x2> {
        match *self {
            NibSliceMut::Full(ref mut s) => s.iter_mut(),
            NibSliceMut::NoL(ref mut s) => s.iter_mut(),
            NibSliceMut::NoR(ref mut s) => s.iter_mut(),
            NibSliceMut::NoBoth(ref mut s) => s.iter_mut(),
        }
    }
}
impl<'a> NibSliceExt for NibSliceMut<'a> {}
impl<'a> NibSliceMutExt for NibSliceMut<'a> {}
