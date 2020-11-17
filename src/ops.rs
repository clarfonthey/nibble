use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{Not, BitAnd, BitOr, BitXor, Shl, Shr};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
use crate::base::{u4lo, u4hi, u4};
use crate::common::{has_higher, ToLo};
use crate::pair::u4x2;

macro_rules! do_ref {
    (
        () (
            $($rest:tt)*
        )
    ) => {};
    (
        (
            $($rest:tt)*
        ) ()
    ) => {};
    (
        (
            $lhs:ident + $rhs:ident;
            $($rest1:tt)*
        ) (
            $tr_op:ident :: $fn_op:ident, $tr_assign:ident :: $fn_assign:ident;
            $($rest2:tt)*
        )
     ) => {
        impl<'rhs> $tr_op<&'rhs $rhs> for $lhs {
            type Output = $lhs;
            fn $fn_op(self, rhs: &'rhs $rhs) -> $lhs {
                self.$fn_op(*rhs)
            }
        }
        impl<'rhs, 'lhs> $tr_op<&'rhs $rhs> for &'lhs $lhs {
            type Output = $lhs;
            fn $fn_op(self, rhs: &'rhs $rhs) -> $lhs {
                (*self).$fn_op(*rhs)
            }
        }
        impl<'lhs> $tr_op<$rhs> for &'lhs $lhs {
            type Output = $lhs;
            fn $fn_op(self, rhs: $rhs) -> $lhs {
                (*self).$fn_op(rhs)
            }
        }
        impl $tr_assign<$rhs> for $lhs {
            fn $fn_assign(&mut self, rhs: $rhs) {
                *self = self.$fn_op(rhs);
            }
        }
        impl<'rhs> $tr_assign<&'rhs $rhs> for $lhs {
            fn $fn_assign(&mut self, rhs: &'rhs $rhs) {
                self.$fn_assign(*rhs)
            }
        }
        do_ref! {
            ($lhs + $rhs;)
            ($($rest2)*)
        }
        do_ref! {
            ($($rest1)*)
            ($tr_op::$fn_op, $tr_assign::$fn_assign; $($rest2)*)
        }
    }
}
do_ref! {
    (
        u4hi + u4hi;
        u4lo + u4lo;
    ) (
        Add::add, AddAssign::add_assign;
        Sub::sub, SubAssign::sub_assign;
        Mul::mul, MulAssign::mul_assign;
        Div::div, DivAssign::div_assign;
        Rem::rem, RemAssign::rem_assign;
        BitAnd::bitand, BitAndAssign::bitand_assign;
        BitOr::bitor, BitOrAssign::bitor_assign;
        BitXor::bitxor, BitXorAssign::bitxor_assign;
    )
}
do_ref! {
    (
        u4hi + u8;
        u4hi + u16;
        u4hi + u32;
        u4hi + u64;
        u4hi + usize;

        u4lo + u8;
        u4lo + u16;
        u4lo + u32;
        u4lo + u64;
        u4lo + usize;
    ) (
        Shl::shl, ShlAssign::shl_assign;
        Shr::shr, ShrAssign::shr_assign;
    )
}

macro_rules! do_value {
    (
        () (
            $($rest:tt)*
        )
    ) => {};
    (
        (
            $($rest:tt)*
        ) ()
    ) => {};
    (
        (
            $lhs:ident + $rhs:ident;
            $($rest1:tt)*
        ) (
            $tr_op:ident::$fn_op:ident;
            $($rest2:tt)*
        )
    ) => {
        impl $tr_op<$rhs> for $lhs {
            type Output = $lhs;
            fn $fn_op(self, rhs: $rhs) -> $lhs {
                let val = self.to_lo().$fn_op(rhs.to_lo());
                if cfg!(debug_assertions) && has_higher(val) {
                    panic!("operation overflowed");
                }
                Self::from_lo(val)
            }
        }
        do_value! {
            ($lhs + $rhs;)
            ($($rest2)*)
        }
        do_value! {
            ($($rest1)*)
            ($tr_op::$fn_op; $($rest2)*)
        }
    }
}
do_value! {
    (
        u4hi + u4hi;
        u4lo + u4lo;
    ) (
        Add::add;
        Sub::sub;
        Mul::mul;
        Div::div;
        Rem::rem;
        BitAnd::bitand;
        BitOr::bitor;
        BitXor::bitxor;
    )
}
do_value! {
    (
        u4hi + u8;
        u4hi + u16;
        u4hi + u32;
        u4hi + u64;
        u4hi + usize;

        u4lo + u8;
        u4lo + u16;
        u4lo + u32;
        u4lo + u64;
        u4lo + usize;
    ) (
        Shl::shl;
        Shr::shr;
    )
}

impl Not for u4hi {
    type Output = u4hi;
    fn not(self) -> u4hi {
        u4hi::from_hi(!self.to_hi())
    }
}
impl Not for u4lo {
    type Output = u4lo;
    fn not(self) -> u4lo {
        u4lo::from_lo(!self.to_lo())
    }
}
impl<'a> Not for &'a u4hi {
    type Output = u4hi;
    fn not(self) -> u4hi {
        (*self).not()
    }
}
impl<'a> Not for &'a u4lo {
    type Output = u4lo;
    fn not(self) -> u4lo {
        (*self).not()
    }
}
impl BitOr<u4lo> for u4hi {
    type Output = u4x2;
    fn bitor(self, rhs: u4lo) -> u4x2 {
        u4x2::from_both(self, rhs)
    }
}
impl BitOr<u4hi> for u4lo {
    type Output = u4x2;
    fn bitor(self, rhs: u4hi) -> u4x2 {
        u4x2::from_both(rhs, self)
    }
}
impl<'rhs> BitOr<&'rhs u4lo> for u4hi {
    type Output = u4x2;
    fn bitor(self, rhs: &'rhs u4lo) -> u4x2 {
        u4x2::from_both(self, *rhs)
    }
}
impl<'rhs> BitOr<&'rhs u4hi> for u4lo {
    type Output = u4x2;
    fn bitor(self, rhs: &'rhs u4hi) -> u4x2 {
        u4x2::from_both(*rhs, self)
    }
}
impl<'lhs> BitOr<u4lo> for &'lhs u4hi {
    type Output = u4x2;
    fn bitor(self, rhs: u4lo) -> u4x2 {
        u4x2::from_both(*self, rhs)
    }
}
impl<'lhs> BitOr<u4hi> for &'lhs u4lo {
    type Output = u4x2;
    fn bitor(self, rhs: u4hi) -> u4x2 {
        u4x2::from_both(rhs, *self)
    }
}
impl<'rhs, 'lhs> BitOr<&'rhs u4lo> for &'lhs u4hi {
    type Output = u4x2;
    fn bitor(self, rhs: &'rhs u4lo) -> u4x2 {
        u4x2::from_both(*self, *rhs)
    }
}
impl<'rhs, 'lhs> BitOr<&'rhs u4hi> for &'lhs u4lo {
    type Output = u4x2;
    fn bitor(self, rhs: &'rhs u4hi) -> u4x2 {
        u4x2::from_both(*rhs, *self)
    }
}
