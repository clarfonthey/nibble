use core::fmt;
use core::str::FromStr;

use arrayvec::{Array};

use base::{u4, u4hi, u4lo};
use base::{ParseNibbleError};
use pair::u4x2;
use vec::NibVec;
use array::{NibArrayVec, NibArray, NibArrayOdd, NibArrayEven};
use slice::{NibSliceFull, NibSliceNoL, NibSliceNoR, NibSliceNoBoth};
use slice::{NibSliceAligned, NibSliceAlignedMut, NibSliceUnaligned, NibSliceUnalignedMut};
use slice::{NibSliceEven, NibSliceEvenMut, NibSliceOdd, NibSliceOddMut};
use slice::{NibSlice, NibSliceMut, NibSliceExt};

macro_rules! do_nibble {
    ($($t:ident)*) => {
        $(
            impl fmt::Binary for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.pad_integral(true, "0b", &self.to_binary())
                }
            }
            impl fmt::Octal for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.pad_integral(true, "0o", &self.to_octal())
                }
            }
            impl fmt::LowerHex for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.pad_integral(true, "0x", &self.to_lower_hex())
                }
            }
            impl fmt::UpperHex for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.pad_integral(true, "0x", &self.to_upper_hex())
                }
            }
            impl fmt::Debug for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.pad_integral(true, "", &self.to_decimal())
                }
            }
            impl fmt::Display for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.pad_integral(true, "", &self.to_decimal())
                }
            }

            /// Parses a string into a nibble.
            ///
            /// This assumes that single digits are in hexadecimal, and larger strings are in
            /// decimal. For more control, use `from_str_radix` instead.
            impl FromStr for $t {
                type Err = ParseNibbleError;
                fn from_str(s: &str) -> Result<Self, ParseNibbleError> {
                    if s.chars().nth(1).is_some() {
                        Self::from_str_radix(s, 10)
                    } else {
                        Self::from_str_radix(s, 16)
                    }
                }
            }
        )*
    }
}

do_nibble! { u4hi u4lo }

impl fmt::Binary for u4x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(true, "0b", &self.to_padded_binary())
    }
}
impl fmt::LowerHex for u4x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(true, "0x", &self.to_padded_lower_hex())
    }
}
impl fmt::UpperHex for u4x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(true, "0x", &self.to_padded_upper_hex())
    }
}
impl fmt::Debug for u4x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("u4x2")
            .field("hi", self.hi())
            .field("lo", self.lo())
            .finish()
    }
}
impl fmt::Display for u4x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(true, "", &self.to_padded_lower_hex())
    }
}

impl NibVec {
    pub(crate) fn try_push<T: u4>(&mut self, nib: T) -> Result<(), ParseNibbleError> {
        Ok(self.push(nib))
    }
}

macro_rules! do_slice {
    ($(
        ($($gen:tt)*)
        ($t:ty)
    ;)*) => {
        $(
            impl<$($gen)*> fmt::Binary for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    // TODO: remove allocation here
                    let mut s: String = String::new();
                    for bin in self.nibbles().map(|nib| nib.to_u4lo().to_padded_binary()) {
                        s.push_str(&*bin);
                    }
                    f.pad_integral(true, "0b", &s)
                }
            }
            impl<$($gen)*> fmt::LowerHex for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    // TODO: remove allocation here
                    let s: String = self.nibbles()
                        .map(|nib| nib.to_u4lo().to_lower_digit())
                        .collect();
                    f.pad_integral(true, "0x", &s)
                }
            }
            impl<$($gen)*> fmt::UpperHex for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    // TODO: remove allocation here
                    let s: String = self.nibbles()
                        .map(|nib| nib.to_u4lo().to_upper_digit())
                        .collect();
                    f.pad_integral(true, "0x", &s)
                }
            }
            impl<$($gen)*> fmt::Debug for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.debug_list()
                        .entries(self.nibbles())
                        .finish()
                }
            }
        )*
    }
}

do_slice! {
    () (NibSliceFull);
    () (NibSliceNoL);
    () (NibSliceNoR);
    () (NibSliceNoBoth);
    ('a) (NibSlice<'a>);
    ('a) (NibSliceMut<'a>);
    ('a) (NibSliceAligned<'a>);
    ('a) (NibSliceAlignedMut<'a>);
    ('a) (NibSliceUnaligned<'a>);
    ('a) (NibSliceUnalignedMut<'a>);
    ('a) (NibSliceEven<'a>);
    ('a) (NibSliceEvenMut<'a>);
    ('a) (NibSliceOdd<'a>);
    ('a) (NibSliceOddMut<'a>);
    (A: Array<Item = u4x2>) (NibArrayOdd<A>);
    (A: Array<Item = u4x2>) (NibArrayEven<A>);
    (A: Array<Item = u4x2>) (NibArray<A>);
}

macro_rules! do_array {
    ($(
        ($($gen:tt)*)
        ($t:path)
    ;)*) => {
        do_slice! { $( ($($gen)*) ($t); )* }

        $(
            impl<$($gen)*> $t {
                /// Converts an ASCII hex string into a nibble vector.
                pub fn from_ascii(s: &[u8]) -> Result<Self, ParseNibbleError> {
                    let mut ret = Self::new();
                    for &b in s {
                        let nib = u4lo::from_ascii_digit(b).ok_or(ParseNibbleError::BadFormat)?;
                        ret.try_push(nib).map_err(|_| ParseNibbleError::TooLarge)?;
                    }
                    Ok(ret)
                }

                /// Converts a hex string into a nibble vector.
                pub fn from_str(s: &str) -> Result<Self, ParseNibbleError> {
                    let mut ret = Self::new();
                    for c in s.chars() {
                        let nib = u4lo::from_digit(c).ok_or(ParseNibbleError::BadFormat)?;
                        ret.try_push(nib).map_err(|_| ParseNibbleError::TooLarge)?;
                    }
                    Ok(ret)
                }
            }

            impl<$($gen)*> FromStr for $t {
                type Err = ParseNibbleError;
                fn from_str(s: &str) -> Result<Self, ParseNibbleError> {
                    Self::from_str(s)
                }
            }
        )*
    }
}

do_array! {
    (A: Array<Item = u4x2>) (NibArrayVec<A>);
    () (NibVec);
}
