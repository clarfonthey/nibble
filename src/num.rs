use num_traits::*;
use crate::base::{u4, u4hi, u4lo};
use crate::common::has_higher;
use crate::base::ParseNibbleError;

macro_rules! do_checked {
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
            $t:ident;
            $($rest1:tt)*
        ) (
            $tr_op:ident :: $fn_op:ident;
            $($rest2:tt)*
        )
    ) => {
        impl $tr_op for $t {
            fn $fn_op(&self, rhs: &$t) -> Option<$t> {
                match self.to_lo().$fn_op(rhs.to_lo()) {
                    Some(x) if !has_higher(x) => Some($t::from_lo(x)),
                    _ => None,
                }
            }
        }
        do_checked! {
            ($t;)
            ($($rest2)*)
        }
        do_checked! {
            ($($rest1)*)
            ($tr_op::$fn_op; $($rest2)*)
        }
    }
}
do_checked! {
    (u4hi; u4lo;) (
        CheckedAdd::checked_add;
        CheckedSub::checked_sub;
        CheckedMul::checked_mul;
        CheckedDiv::checked_div;
    )
}

macro_rules! do_wrapping {
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
            $t:ident;
            $($rest1:tt)*
        ) (
            $tr_op:ident :: $fn_op:ident;
            $($rest2:tt)*
        )
    ) => {
        impl $tr_op for $t {
            fn $fn_op(&self, rhs: &$t) -> $t {
                $t::from_lo(self.to_lo().$fn_op(rhs.to_lo()))
            }
        }
        do_wrapping! {
            ($t;)
            ($($rest2)*)
        }
        do_wrapping! {
            ($($rest1)*)
            ($tr_op::$fn_op; $($rest2)*)
        }
    }
}
do_wrapping! {
    (u4hi; u4lo;) (
        WrappingAdd::wrapping_add;
        WrappingSub::wrapping_sub;
        WrappingMul::wrapping_mul;
    )
}

macro_rules! do_extras {
    ($($t:ident)*) => {
        $(
            impl Saturating for $t {
                fn saturating_add(self, rhs: $t) -> $t {
                    self.checked_add(&rhs).unwrap_or($t::max_value())
                }
                fn saturating_sub(self, rhs: $t) -> $t {
                    self.checked_sub(&rhs).unwrap_or($t::min_value())
                }
            }
            impl Bounded for $t {
                fn min_value() -> $t {
                    $t::from_lo(0b0000)
                }
                fn max_value() -> $t {
                    $t::from_lo(0b1111)
                }
            }
            impl Zero for $t {
                fn zero() -> $t {
                    $t::from_lo(0)
                }
                fn is_zero(&self) -> bool {
                    *self == $t::zero()
                }
            }
            impl One for $t {
                fn one() -> $t {
                    $t::from_lo(1)
                }
            }
            impl Unsigned for $t {}
            impl FromPrimitive for $t {
                fn from_i64(n: i64) -> Option<$t> {
                    match n.to_u8() {
                        Some(n) => $t::try_from_lo(n),
                        None => None,
                    }
                }
                fn from_u64(n: u64) -> Option<$t> {
                    match n.to_u8() {
                        Some(n) => $t::try_from_lo(n),
                        None => None,
                    }
                }
            }
            impl NumCast for $t {
                fn from<P: ToPrimitive>(n: P) -> Option<$t> {
                    match n.to_u8() {
                        Some(m) => $t::try_from_lo(m),
                        None => None,
                    }
                }
            }
            impl ToPrimitive for $t {
                fn to_i64(&self) -> Option<i64> {
                    Some(self.to_lo() as i64)
                }
                fn to_u64(&self) -> Option<u64> {
                    Some(self.to_lo() as u64)
                }
            }
            impl PrimInt for $t {
                fn count_ones(self) -> u32 {
                    self.to_lo().count_ones()
                }
                fn count_zeros(self) -> u32 {
                    self.to_lo().count_zeros() - 4
                }
                fn leading_zeros(self) -> u32 {
                    self.to_lo().leading_zeros() - 4
                }
                fn trailing_zeros(self) -> u32 {
                    self.to_hi().trailing_zeros() - 4
                }
                fn rotate_left(self, n: u32) -> $t {
                    $t::from_repeated(self.to_repeated().rotate_left(n))
                }
                fn rotate_right(self, n: u32) -> $t {
                    $t::from_repeated(self.to_repeated().rotate_right(n))
                }
                fn signed_shl(self, n: u32) -> $t {
                    $t::from_lo(self.to_lo().signed_shl(n))
                }
                fn signed_shr(self, n: u32) -> $t {
                    $t::from_lo(self.to_lo().signed_shr(n))
                }
                fn unsigned_shl(self, n: u32) -> $t {
                    $t::from_lo(self.to_lo().unsigned_shl(n))
                }
                fn unsigned_shr(self, n: u32) -> $t {
                    $t::from_lo(self.to_lo().unsigned_shr(n))
                }
                fn swap_bytes(self) -> $t {
                    self
                }
                fn from_be(x: Self) -> $t {
                    x
                }
                fn from_le(x: Self) -> $t {
                    x
                }
                fn to_be(self) -> $t {
                    self
                }
                fn to_le(self) -> $t {
                    self
                }
                fn pow(self, exp: u32) -> $t {
                    let pow = self.to_lo().pow(exp);
                    if cfg!(debug_assertions) && pow & 0xF0 != 0 {
                        panic!("overflow");
                    }
                    $t::from_lo(pow)
                }
            }
            impl Num for $t {
                type FromStrRadixErr = ParseNibbleError;
                fn from_str_radix(s: &str, radix: u32) -> Result<$t, ParseNibbleError> {
                    u4::from_str_radix(s, radix)
                }
            }
        )*
    }
}
do_extras! { u4hi u4lo }

#[cfg(test)]
mod tests {
    use crate::base::{u4hi, u4lo, u4};
    use num_traits::*;

    #[test]
    fn num_traits() {
        fn verify<T: u4 + Num + NumAssign + NumAssignRef + NumRef + RefNum<T>>(_: T) {}
        verify(u4hi::zero());
        verify(u4lo::zero());
    }
}
