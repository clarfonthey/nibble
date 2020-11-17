//! Types for manipulating pairs of nibbles in a single byte.
use arrayvec::ArrayString;

use crate::base::{u4, u4hi, u4lo};
use core::{cell, fmt};

/// A `u8` split into its component nibbles.
#[derive(Clone, Copy)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub union u4x2 {
    hi: u4hi,
    lo: u4lo,
    byte: u8,
}
impl u4x2 {
    /// Creates a pair with an empty low-order nibble.
    pub fn from_hi(hi: u4hi) -> Self {
        Self { byte: hi.to_hi() }
    }

    /// Creates a pair with an empty high-order nibble.
    pub fn from_lo(lo: u4lo) -> Self {
        Self { byte: lo.to_lo() }
    }

    /// Creates a pair from its components.
    #[inline(always)]
    pub fn from_both(hi: u4hi, lo: u4lo) -> Self {
        Self { byte: hi.to_hi() & lo.to_lo() }
    }

    /// Creates a pair from an already-combined byte.
    #[inline(always)]
    pub fn from_byte(byte: u8) -> u4x2 {
        u4x2 { byte }
    }

    /// The high-order nibble.
    #[inline(always)]
    pub fn hi(&self) -> &u4hi {
        unsafe { &self.hi }
    }

    /// The low-order nibble.
    #[inline(always)]
    pub fn lo(&self) -> &u4lo {
        unsafe { &self.lo }
    }

    /// Both nibbles.
    #[inline(always)]
    pub fn both(&self) -> (&u4hi, &u4lo) {
        (self.hi(), self.lo())
    }

    /// Both nibbles as a byte.
    #[inline(always)]
    pub fn byte(&self) -> &u8 {
        unsafe { &self.byte }
    }

    /// Mutable access to the high-order nibble in a cell.
    #[inline(always)]
    pub fn hi_mut(&mut self) -> &U4HiCell {
        unsafe { &*(self as *const u4x2 as *const U4HiCell) }
    }

    /// Mutable access to the low-order nibble in a cell.
    #[inline(always)]
    pub fn lo_mut(&mut self) -> &U4LoCell {
        unsafe { &*(self as *const u4x2 as *const U4LoCell) }
    }

    /// Mutable access to both nibbles in cells.
    #[inline(always)]
    pub fn both_mut(&mut self) -> (&U4HiCell, &U4LoCell) {
        let hi = unsafe { &*(self as *const u4x2 as *const U4HiCell) };
        let lo = unsafe { &*(self as *const u4x2 as *const U4LoCell) };
        (hi, lo)
    }

    /// Mutable access to the whole byte.
    #[inline(always)]
    pub fn byte_mut(&mut self) -> &mut u8 {
        unsafe { &mut self.byte }
    }

    /// Provides access to the nibbles in a byte.
    #[inline(always)]
    pub fn from_ref(byte: &u8) -> &u4x2 {
        unsafe { &*(byte as *const u8 as *const u4x2) }
    }

    /// Provides access to the nibbles in a byte.
    #[inline(always)]
    pub fn from_mut(byte: &mut u8) -> &mut u4x2 {
        unsafe { &mut *(byte as *mut u8 as *mut u4x2) }
    }

    /// Sets the low-order nibble.
    pub fn set_lo<T: u4>(&mut self, lo: T) {
        self.byte = unsafe { self.hi.to_hi() } | lo.to_lo();
    }

    /// Sets the low-order nibble.
    pub fn set_hi<T: u4>(&mut self, hi: T) {
        self.byte = unsafe { self.lo.to_lo() } | hi.to_hi();
    }

    /// Swaps the nibbles in the pair.
    pub fn swap_pairs(&mut self) {
        self.byte = self.hi().to_lo() | self.lo().to_hi();
    }

    /// Iterator over the nibble pair.
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter { hi: Some(self.hi()), lo: Some(self.lo()) }
    }

    /// Mutable iterator over the nibble pair.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut {
        let (hi, lo) = self.both_mut();
        IterMut { hi: Some(hi), lo: Some(lo) }
    }

    /// Converts an ASCII hex digit into a nibble.
    pub fn from_ascii_digits(lo: u8, hi: u8) -> Option<u4x2> {
        if let Some(hi) = u4hi::from_ascii_digit(hi) {
            if let Some(lo) = u4lo::from_ascii_digit(lo) {
                return Some(hi | lo);
            }
        }
        None
    }

    /// Converts a nibble into a lowercase ASCII hex digit.
    pub fn to_lower_ascii_digits(&self) -> (u8, u8) {
        (self.hi().to_lower_ascii_digit(), self.lo().to_lower_ascii_digit())
    }

    /// Converts a nibble into an uppercase ASCII hex digit.
    pub fn to_upper_ascii_digit(&self) -> (u8, u8) {
        (self.hi().to_upper_ascii_digit(), self.lo().to_upper_ascii_digit())
    }

    /// Converts a hex digit into a nibble.
    pub fn from_digits(lo: char, hi: char) -> Option<u4x2> {
        if let Some(hi) = u4hi::from_digit(hi) {
            if let Some(lo) = u4lo::from_digit(lo) {
                return Some(hi | lo);
            }
        }
        None
    }

    /// Converts a nibble into a lowercase hex digit.
    pub fn to_lower_digits(&self) -> (char, char) {
        (self.hi().to_lower_digit(), self.lo().to_lower_digit())
    }

    /// Converts a nibble into an uppercase hex digit.
    pub fn to_upper_digits(&self) -> (char, char) {
        (self.hi().to_upper_digit(), self.lo().to_upper_digit())
    }

    /// Converts a nibble into a lowercase hex string.
    pub fn to_padded_lower_hex(&self) -> ArrayString<[u8; 2]> {
        let mut s = ArrayString::new();
        s.push(self.hi().to_lower_digit());
        s.push(self.lo().to_lower_digit());
        s
    }

    /// Converts a nibble into an uppercase hex string.
    pub fn to_padded_upper_hex(&self) -> ArrayString<[u8; 2]> {
        let mut s = ArrayString::new();
        s.push(self.hi().to_upper_digit());
        s.push(self.lo().to_upper_digit());
        s
    }

    /// Converts a nibble into a binary string.
    pub fn to_padded_binary(&self) -> ArrayString<[u8; 8]> {
        let mut s = ArrayString::new();
        s.push_str(&self.hi().to_padded_binary());
        s.push_str(&self.lo().to_padded_binary());
        s
    }
}
impl From<u4hi> for u4x2 {
    fn from(hi: u4hi) -> u4x2 {
        Self::from_hi(hi)
    }
}
impl From<u4lo> for u4x2 {
    fn from(lo: u4lo) -> u4x2 {
        Self::from_lo(lo)
    }
}
impl From<u8> for u4x2 {
    fn from(byte: u8) -> u4x2 {
        Self::from_byte(byte)
    }
}
impl From<u4x2> for u4hi {
    fn from(pair: u4x2) -> u4hi {
        *pair.hi()
    }
}
impl From<u4x2> for u4lo {
    fn from(pair: u4x2) -> u4lo {
        *pair.lo()
    }
}
impl From<u4x2> for u8 {
    fn from(pair: u4x2) -> u8 {
        *pair.byte()
    }
}

/// Iterator over the nibbles in a pair.
#[derive(Clone, Debug)]
pub struct Iter<'a> {
    hi: Option<&'a dyn u4>,
    lo: Option<&'a dyn u4>,
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a dyn u4;
    fn next(&mut self) -> Option<&'a dyn u4> {
        self.hi.take().or_else(|| self.lo.take())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}
impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<&'a dyn u4> {
        self.lo.take().or_else(|| self.hi.take())
    }
}
impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.hi.is_some() as usize + self.lo.is_some() as usize
    }
}

/// Mutable iterator over the nibbles in a pair.
#[derive(Clone, Debug)]
pub struct IterMut<'a> {
    hi: Option<&'a dyn U4Cell>,
    lo: Option<&'a dyn U4Cell>,
}
impl<'a> Iterator for IterMut<'a> {
    type Item = &'a dyn U4Cell;
    fn next(&mut self) -> Option<&'a dyn U4Cell> {
        self.hi.take().or_else(|| self.lo.take())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}
impl<'a> DoubleEndedIterator for IterMut<'a> {
    fn next_back(&mut self) -> Option<&'a dyn U4Cell> {
        self.lo.take().or_else(|| self.hi.take())
    }
}
impl<'a> ExactSizeIterator for IterMut<'a> {
    fn len(&self) -> usize {
        self.hi.is_some() as usize + self.lo.is_some() as usize
    }
}

/// A generic cell which contains a nibble.
pub trait U4Cell: fmt::Debug {
    /// Gets a high-order version of the nibble in this cell.
    fn get_hi(&self) -> u4hi;

    /// Gets a low-order version of the nibble in this cell.
    fn get_lo(&self) -> u4lo;

    /// Sets the value of the nibble in this cell, given a high-order nibble.
    fn set_from_hi(&self, hi: u4hi);

    /// Sets the value of the nibble in this cell, given a low-order nibble.
    fn set_from_lo(&self, lo: u4lo);

    /// Sets the value of the nibble in this cell, given a generic nibble.
    ///
    /// Note that this is preferred, unless trait objects are used.
    fn set<T: u4>(&self, nib: T) where Self: Sized;

    /// Swaps the nibble with the value of another nibble.
    fn swap(&self, nib: &dyn U4Cell);
}

/// A cell for mutating a high-order nibble.
#[derive(Clone)]
#[repr(C)]
pub struct U4HiCell {
    inner: cell::Cell<u4x2>,
}
impl U4Cell for U4HiCell {
    #[inline]
    fn get_hi(&self) -> u4hi {
        *self.inner.get().hi()
    }
    #[inline]
    fn get_lo(&self) -> u4lo {
        self.inner.get().hi().to_u4lo()
    }
    #[inline]
    fn set_from_hi(&self, hi: u4hi) {
        unsafe { &mut *self.inner.as_ptr() }.set_hi(hi)
    }
    #[inline]
    fn set_from_lo(&self, lo: u4lo) {
        unsafe { &mut *self.inner.as_ptr() }.set_hi(lo)
    }
    #[inline]
    fn set<T: u4>(&self, nib: T) {
        unsafe { &mut *self.inner.as_ptr() }.set_hi(nib)
    }
    #[inline]
    fn swap(&self, nib: &dyn U4Cell) {
        let hi = self.get_hi();
        self.set_from_hi(nib.get_hi());
        nib.set_from_hi(hi);
    }
}
impl fmt::Debug for U4HiCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.get_hi(), f)
    }
}

/// A cell for mutating a low-order nibble.
#[derive(Clone)]
#[repr(C)]
pub struct U4LoCell {
    inner: cell::Cell<u4x2>,
}
impl U4Cell for U4LoCell {
    #[inline]
    fn get_hi(&self) -> u4hi {
        self.inner.get().lo().to_u4hi()
    }
    #[inline]
    fn get_lo(&self) -> u4lo {
        *self.inner.get().lo()
    }
    #[inline]
    fn set_from_hi(&self, hi: u4hi) {
        unsafe { &mut *self.inner.as_ptr() }.set_lo(hi)
    }
    #[inline]
    fn set_from_lo(&self, lo: u4lo) {
        unsafe { &mut *self.inner.as_ptr() }.set_lo(lo)
    }
    #[inline]
    fn set<T: u4>(&self, nib: T) {
        unsafe { &mut *self.inner.as_ptr() }.set_lo(nib)
    }
    #[inline]
    fn swap(&self, nib: &dyn U4Cell) {
        let lo = self.get_lo();
        self.set_from_lo(nib.get_lo());
        nib.set_from_lo(lo);
    }
}
impl fmt::Debug for U4LoCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.get_lo(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u4cell_works() {
        let mut byte = u4x2::from_byte(0x13);
        byte.lo_mut() as &dyn U4Cell;
        byte.hi_mut() as &dyn U4Cell;
    }

    #[test]
    fn set() {
        let mut byte = u4x2::from_byte(0x13);
        assert_eq!(*byte.hi(), 1);
        assert_eq!(*byte.lo(), 3);
        byte.set_lo(u4lo::from_lo(2));
        assert_eq!(*byte.hi(), 1);
        assert_eq!(*byte.lo(), 2);
        byte.set_hi(u4hi::from_lo(4));
        assert_eq!(*byte.hi(), 4);
        assert_eq!(*byte.lo(), 2);
    }

    #[test]
    fn cell_set() {
        let mut byte = u4x2::from_byte(0x13);
        let (hi, lo) = byte.both_mut();
        assert_eq!(hi.get_hi(), 1);
        assert_eq!(lo.get_lo(), 3);
        lo.set(u4lo::from_lo(2));
        assert_eq!(hi.get_hi(), 1);
        assert_eq!(lo.get_lo(), 2);
        hi.set(u4hi::from_lo(4));
        assert_eq!(hi.get_hi(), 4);
        assert_eq!(lo.get_lo(), 2);
    }
}
