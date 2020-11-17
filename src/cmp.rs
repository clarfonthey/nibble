use arrayvec::Array;

use crate::array::{NibArray, NibArrayEven, NibArrayOdd, NibArrayVec};
use crate::base::{u4hi, u4lo, u4};
use core::{cmp, hash};
use crate::pair::u4x2;
use crate::slice::{NibSliceFull, NibSliceNoL, NibSliceNoR, NibSliceNoBoth};
use crate::slice::{NibSliceAligned, NibSliceAlignedMut, NibSliceUnaligned, NibSliceUnalignedMut};
use crate::slice::{NibSliceEvenMut, NibSliceEven, NibSliceOdd, NibSliceOddMut};
use crate::slice::{NibSliceExt, NibSlice, NibSliceMut};
use crate::slice::private::Sealed;
use crate::vec::NibVec;

macro_rules! do_impl {
    ($($t:ident)*) => {
        $(
            impl PartialEq<u4hi> for $t {
                fn eq(&self, rhs: &u4hi) -> bool {
                    self.to_lo() == rhs.to_lo()
                }
            }
            impl PartialEq<u4lo> for $t {
                fn eq(&self, rhs: &u4lo) -> bool {
                    self.to_lo() == rhs.to_lo()
                }
            }
            impl PartialEq<u8> for $t {
                fn eq(&self, rhs: &u8) -> bool {
                    self.to_lo() == *rhs
                }
            }
            impl PartialEq<$t> for u8 {
                fn eq(&self, rhs: &$t) -> bool {
                    *self == rhs.to_lo()
                }
            }
            impl PartialOrd<u4hi> for $t {
                fn partial_cmp(&self, rhs: &u4hi) -> Option<cmp::Ordering> {
                    self.to_lo().partial_cmp(&rhs.to_lo())
                }
            }
            impl PartialOrd<u4lo> for $t {
                fn partial_cmp(&self, rhs: &u4lo) -> Option<cmp::Ordering> {
                    self.to_lo().partial_cmp(&rhs.to_lo())
                }
            }
            impl PartialOrd<u8> for $t {
                fn partial_cmp(&self, rhs: &u8) -> Option<cmp::Ordering> {
                    self.to_lo().partial_cmp(rhs)
                }
            }
            impl PartialOrd<$t> for u8 {
                fn partial_cmp(&self, rhs: &$t) -> Option<cmp::Ordering> {
                    self.partial_cmp(&rhs.to_lo())
                }
            }
            impl hash::Hash for $t {
                fn hash<H: hash::Hasher>(&self, state: &mut H) {
                    self.to_lo().hash(state)
                }
            }
            impl Eq for $t {}
            impl Ord for $t {
                fn cmp(&self, rhs: &$t) -> cmp::Ordering {
                    self.to_lo().cmp(&rhs.to_lo())
                }
            }
        )*
    }
}

do_impl! { u4hi u4lo }

impl PartialEq<u4x2> for u4x2 {
    fn eq(&self, rhs: &u4x2) -> bool {
        self.byte() == rhs.byte()
    }
}
impl PartialEq<u8> for u4x2 {
    fn eq(&self, rhs: &u8) -> bool {
        self.byte() == rhs
    }
}
impl PartialEq<u4x2> for u8 {
    fn eq(&self, rhs: &u4x2) -> bool {
        self == rhs.byte()
    }
}
impl PartialOrd<u4x2> for u4x2 {
    fn partial_cmp(&self, rhs: &u4x2) -> Option<cmp::Ordering> {
        self.byte().partial_cmp(rhs.byte())
    }
}
impl PartialOrd<u8> for u4x2 {
    fn partial_cmp(&self, rhs: &u8) -> Option<cmp::Ordering> {
        self.byte().partial_cmp(rhs)
    }
}
impl PartialOrd<u4x2> for u8 {
    fn partial_cmp(&self, rhs: &u4x2) -> Option<cmp::Ordering> {
        self.partial_cmp(rhs.byte())
    }
}
impl hash::Hash for u4x2 {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.byte().hash(state)
    }
}
impl Eq for u4x2 {}
impl Ord for u4x2 {
    fn cmp(&self, rhs: &u4x2) -> cmp::Ordering {
        self.byte().cmp(rhs.byte())
    }
}

macro_rules! do_slice {
    ($(
        ($($gen:tt)*)
        ($t:path)
    ;)*) => {
        $(
            impl<$($gen)*, Rhs: ?Sized + NibSliceExt> PartialEq<Rhs> for $t {
                fn eq(&self, rhs: &Rhs) -> bool {
                    if self.len() != rhs.len() {
                        return false
                    }

                    let cond =
                        self.has_left_hi() == rhs.has_left_hi() &&
                        self.has_right_lo() == rhs.has_right_lo();
                    if cond {
                        self.decompose() == rhs.decompose()
                    } else {
                        self.nibbles().map(u4::to_lo).eq(rhs.nibbles().map(u4::to_lo))
                    }
                }
            }
            impl<$($gen)*, Rhs: ?Sized + NibSliceExt> PartialOrd<Rhs> for $t {
                fn partial_cmp(&self, rhs: &Rhs) -> Option<cmp::Ordering> {
                    let cond =
                        self.has_left_hi() == rhs.has_left_hi() &&
                        self.has_right_lo() == rhs.has_right_lo();
                    if cond {
                        self.decompose().partial_cmp(&rhs.decompose())
                    } else {
                        self.nibbles().map(u4::to_lo).partial_cmp(rhs.nibbles().map(u4::to_lo))
                    }
                }
            }
            impl<$($gen)*> hash::Hash for $t {
                fn hash<H: hash::Hasher>(&self, state: &mut H) {
                    self.decompose().hash(state)
                }
            }
            impl<$($gen)*> Eq for $t {}
            impl<$($gen)*> Ord for $t {
                fn cmp(&self, rhs: &$t) -> cmp::Ordering {
                    self.decompose().cmp(&rhs.decompose())
                }
            }
        )*
    }
}

do_slice! {
    ('unused) (NibSliceFull);
    ('unused) (NibSliceNoL);
    ('unused) (NibSliceNoR);
    ('unused) (NibSliceNoBoth);
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
    (A: Array<Item = u4x2>) (NibArrayVec<A>);
    ('unused) (NibVec);
}
