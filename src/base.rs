//! Basic nibble types.
use arrayvec::{ArrayString, ArrayVec};

use core::fmt;
use common::{higher_to_higher, higher_to_lower, lower_to_higher, lower_to_lower};
use common::{has_higher, has_lower};
use common::{bits, octal_digits, decimal_digits};


/// A nibble.
#[allow(non_camel_case_types)]
pub trait u4
    : fmt::Debug
    + fmt::Display
    + fmt::Binary
    + fmt::Octal
    + fmt::LowerHex
    + fmt::UpperHex
    + PartialEq<u4lo>
    + PartialEq<u4hi>
    + PartialOrd<u4lo>
    + PartialOrd<u4hi>
{
    /// Constructs a nibble from the high-order bits the a given byte.
    fn from_hi(lo_and_hi: u8) -> Self where Self: Sized;

    /// Constructs a nibble from the low-order bits the a given byte.
    fn from_lo(lo_and_hi: u8) -> Self where Self: Sized;

    /// Constructs a nibble from a byte with the same data in the high- and low-order bits.
    fn from_repeated(lo_and_hi: u8) -> Self where Self: Sized;

    /// Converts this nibble into a byte with its high-order bits set and low-order bits zero.
    fn to_hi(&self) -> u8;

    /// Converts this nibble into a byte with its low-order bits set and high-order bits zero.
    fn to_lo(&self) -> u8;

    /// Tries to constructs a nibble from the high-order bits the a given byte.
    ///
    /// Fails if the low-order bits are nonzero.
    #[inline]
    fn try_from_hi(lo_and_hi: u8) -> Option<Self> where Self: Sized {
        if has_lower(lo_and_hi) {
            None
        } else {
            Some(Self::from_hi(lo_and_hi))
        }
    }

    /// Tries to constructs a nibble from the low-order bits the a given byte.
    ///
    /// Fails if the high-order bits are nonzero.
    #[inline]
    fn try_from_lo(lo_and_hi: u8) -> Option<Self> where Self: Sized {
        if has_higher(lo_and_hi) {
            None
        } else {
            Some(Self::from_lo(lo_and_hi))
        }
    }

    /// Converts this nibble into a byte with the same data in the high- and low-order bits.
    #[inline]
    fn to_repeated(&self) -> u8 {
        self.to_lo() & self.to_hi()
    }

    /// Converts this nibble into a high-order version.
    #[inline(always)]
    fn to_u4hi(&self) -> u4hi {
        u4hi::from_hi(self.to_hi())
    }

    /// Converts this nibble into a low-order version.
    #[inline(always)]
    fn to_u4lo(&self) -> u4lo {
        u4lo::from_lo(self.to_lo())
    }

    /// Converts an ASCII hex digit into a nibble.
    fn from_ascii_digit(b: u8) -> Option<Self>
    where
        Self: Sized
    {
        match b {
            b'0'...b'9' => Some(Self::from_lo(b - b'0')),
            b'A'...b'F' => Some(Self::from_lo(b - b'A' + 0xA)),
            b'a'...b'f' => Some(Self::from_lo(b - b'a' + 0xa)),
            _ => None,
        }
    }

    /// Converts a nibble into a lowercase ASCII hex digit.
    fn to_lower_ascii_digit(&self) -> u8 {
        let val = self.to_lo();
        match val {
            0x0...0x9 => b'0' + val,
            0xa...0xf => b'a' + val - 0xa,
            _ => unreachable!(),
        }
    }

    /// Converts a nibble into an uppercase ASCII hex digit.
    fn to_upper_ascii_digit(&self) -> u8 {
        let val = self.to_lo();
        match val {
            0x0...0x9 => b'0' + val,
            0xA...0xF => b'A' + val - 0xA,
            _ => unreachable!(),
        }
    }

    /// Converts a hex digit into a nibble.
    fn from_digit(c: char) -> Option<Self>
    where
        Self: Sized
    {
        match c {
            '0'...'9' => Some(Self::from_lo(u32::from(c) as u8 - b'0')),
            'A'...'F' => Some(Self::from_lo(u32::from(c) as u8 - b'A' + 0xA)),
            'a'...'f' => Some(Self::from_lo(u32::from(c) as u8 - b'a' + 0xa)),
            _ => None,
        }
    }

    /// Converts a nibble into a lowercase hex digit.
    fn to_lower_digit(&self) -> char {
        self.to_lower_ascii_digit() as char
    }

    /// Converts a nibble into an uppercase hex digit.
    fn to_upper_digit(&self) -> char {
        self.to_upper_ascii_digit() as char
    }

    /// Converts a nibble into a lowercase hex string.
    fn to_lower_hex(&self) -> ArrayString<[u8; 1]> {
        let mut s = ArrayString::new();
        s.push(self.to_lower_digit());
        s
    }

    /// Converts a nibble into an uppercase hex string.
    fn to_upper_hex(&self) -> ArrayString<[u8; 1]> {
        let mut s = ArrayString::new();
        s.push(self.to_upper_digit());
        s
    }

    /// Converts an ASCII binary string into a nibble.
    fn from_ascii_binary(s: ArrayVec<[u8; 4]>) -> Option<Self>
    where
        Self: Sized
    {
        let mut nib = 0;
        for digit in s {
            nib <<= 1;
            match digit {
                b'0' => (),
                b'1' => nib |= 1,
                _ => return None,
            }
        }
        Some(Self::from_lo(nib))
    }

    /// Converts an ASCII binary string into a nibble.
    fn from_binary(s: ArrayString<[u8; 4]>) -> Option<Self>
    where
        Self: Sized
    {
        let mut nib = 0;
        for digit in s.chars() {
            nib <<= 1;
            match digit {
                '0' => (),
                '1' => nib |= 1,
                _ => return None,
            }
        }
        Some(Self::from_lo(nib))
    }

    /// Converts a nibble into a 4-digit binary string.
    fn to_padded_binary(&self) -> ArrayString<[u8; 4]> {
        let mut s = ArrayString::new();
        for bit in bits(self.to_lo()).iter() {
            s.push((b'0' + bit) as char);
        }
        s
    }

    /// Converts a nibble into a binary string.
    fn to_binary(&self) -> ArrayString<[u8; 4]> {
        let mut s = ArrayString::new();
        for bit in bits(self.to_lo()).iter().skip_while(|&&x| x == 0) {
            s.push((b'0' + bit) as char);
        }
        if s.is_empty() {
            s.push('0');
        }
        s
    }

    /// Converts a nibble into an octal string.
    fn to_octal(&self) -> ArrayString<[u8; 2]> {
        let mut s = ArrayString::new();
        for digit in octal_digits(self.to_lo()).iter().skip_while(|&&x| x == 0) {
            s.push((b'0' + digit) as char);
        }
        if s.is_empty() {
            s.push('0');
        }
        s
    }

    /// Converts a nibble into a decimal string.
    fn to_decimal(&self) -> ArrayString<[u8; 2]> {
        let mut s = ArrayString::new();
        for digit in decimal_digits(self.to_lo()).iter().skip_while(|&&x| x == 0) {
            s.push((b'0' + digit) as char);
        }
        if s.is_empty() {
            s.push('0');
        }
        s
    }

    /// Converts an ASCII string of the given radix into a nibble.
    ///
    /// # Panics
    ///
    /// Panics if `radix > 36`.
    fn from_ascii_radix(s: &[u8], radix: u32) -> Result<Self, ParseNibbleError>
    where
        Self: Sized
    {
        if let Some((&first, rest)) = s.split_first() {
            let mut nib = digit(first, radix)?;
            for &b in rest {
                nib += digit(b, radix)?;
                if has_higher(nib) {
                    return Err(ParseNibbleError::TooLarge)
                }
            }
            Ok(Self::from_lo(nib))
        } else {
            Err(ParseNibbleError::Empty)
        }
    }

    /// Converts a string of the given radix into a nibble.
    ///
    /// # Panics
    ///
    /// Panics if `radix > 36`.
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseNibbleError>
    where
        Self: Sized
    {
        Self::from_ascii_radix(s.as_bytes(), radix)
    }
}

/// A nibble stored in the most significant bits of a byte.
#[derive(Copy, Clone)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub union u4hi {
    hi_and_lo: u8,
}
impl u4 for u4hi {
    #[inline(always)]
    fn from_hi(hi_and_lo: u8) -> Self {
        Self { hi_and_lo }
    }
    #[inline(always)]
    fn from_lo(hi_and_lo: u8) -> Self {
        Self { hi_and_lo: lower_to_higher(hi_and_lo) }
    }
    #[inline(always)]
    fn from_repeated(hi_and_lo: u8) -> Self {
        Self::from_hi(hi_and_lo)
    }
    #[inline(always)]
    fn to_hi(&self) -> u8 {
        unsafe { higher_to_higher(self.hi_and_lo) }
    }
    #[inline(always)]
    fn to_lo(&self) -> u8 {
        unsafe { higher_to_lower(self.hi_and_lo) }
    }
}

/// A nibble stored in the low-order bits of a byte.
#[derive(Copy, Clone)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub union u4lo {
    hi_and_lo: u8,
}
impl u4 for u4lo {
    #[inline(always)]
    fn from_hi(hi_and_lo: u8) -> Self {
        Self { hi_and_lo: higher_to_lower(hi_and_lo) }
    }
    #[inline(always)]
    fn from_lo(hi_and_lo: u8) -> Self {
        Self { hi_and_lo }
    }
    #[inline(always)]
    fn from_repeated(hi_and_lo: u8) -> Self {
        Self::from_lo(hi_and_lo)
    }
    #[inline(always)]
    fn to_hi(&self) -> u8 {
        unsafe { lower_to_higher(self.hi_and_lo) }
    }
    #[inline(always)]
    fn to_lo(&self) -> u8 {
        unsafe { lower_to_lower(self.hi_and_lo) }
    }
}
impl From<u4lo> for u4hi {
    fn from(lo: u4lo) -> u4hi {
        u4hi::from_hi(lo.to_hi())
    }
}
impl From<u4hi> for u4lo {
    fn from(hi: u4hi) -> u4lo {
        u4lo::from_lo(hi.to_lo())
    }
}

/// An error that occurs when parsing a nibble.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ParseNibbleError {
    /// Given string was empty.
    Empty,

    /// Given number was larger than a nibble, or larger than the slice could hold.
    TooLarge,

    /// Given string was not a valid number.
    BadFormat,
}
impl ParseNibbleError {
    /// User-friendly description of the error.
    pub fn description(&self) -> &'static str {
        match *self {
            ParseNibbleError::Empty => "string was empty",
            ParseNibbleError::TooLarge => "number was too large",
            ParseNibbleError::BadFormat => "string was not a valid number",
        }
    }
}
impl fmt::Display for ParseNibbleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.description())
    }
}
#[cfg(feature = "std")]
impl ::std::error::Error for ParseNibbleError {
    fn description(&self) -> &str {
        self.description()
    }
}

pub(crate) fn digit(b: u8, radix: u32) -> Result<u8, ParseNibbleError> {
    match char::from(b).to_digit(radix) {
        None => Err(ParseNibbleError::BadFormat),
        Some(d) => if has_higher(d as u8) {
            Err(ParseNibbleError::TooLarge)
        } else {
            Ok(d as u8)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u4_works() {
        let lo = u4lo::from_lo(3);
        let hi = u4hi::from_lo(5);
        &lo as &u4;
        &hi as &u4;
    }
}
