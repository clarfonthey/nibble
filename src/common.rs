use crate::base::u4;
use crate::pair::{u4x2, U4Cell};

#[inline(always)] pub(crate) fn has_lower(byte: u8) -> bool { byte & 0b0000_1111 != 0 }
#[inline(always)] pub(crate) fn has_higher(byte: u8) -> bool { byte & 0b1111_0000 != 0 }
#[inline(always)] pub(crate) fn lower_to_lower(byte: u8) -> u8 { byte & 0b0000_1111 }
#[inline(always)] pub(crate) fn lower_to_higher(byte: u8) -> u8 { byte << 4 }
#[inline(always)] pub(crate) fn higher_to_lower(byte: u8) -> u8 { byte >> 4 }
#[inline(always)] pub(crate) fn higher_to_higher(byte: u8) -> u8 { byte & 0b1111_0000 }

pub(crate) fn bits(lo: u8) -> [u8; 4] {
    [(lo & 0b1000) >> 3, (lo & 0b0100) >> 2, (lo & 0b0010) >> 1, lo & 0b0001]
}
pub(crate) fn octal_digits(lo: u8) -> [u8; 2] {
    [lo >> 3, lo & 0b111]
}
pub(crate) fn decimal_digits(lo: u8) -> [u8; 2] {
    [lo / 10, lo % 10]
}

pub(crate) fn shift_right(slice: &mut [u4x2], nibidx: usize) {
    let bytelen = slice.len();
    let byteidx = nibidx >> 1;
    let niblen = bytelen << 1;
    assert!(nibidx <= niblen);

    for i in (byteidx + 1..bytelen).rev() {
        *slice[i].byte_mut() >>= 4;
        *slice[i].byte_mut() |= slice[i - 1].lo().to_hi();
    }

    // shifting even nibbles requires moving hi -> lo
    if nibidx & 1 == 0 {
        *slice[byteidx].byte_mut() >>= 4;

    // shifting odd nibbles requires removing lo
    } else {
        *slice[byteidx].byte_mut() &= 0xF0;
    }
}

pub(crate) fn shift_left(slice: &mut [u4x2], nibidx: usize) {
    let bytelen = slice.len();
    let byteidx = nibidx >> 1;
    let niblen = bytelen << 1;
    assert!(nibidx < niblen);

    // shifting even nibbles requires no extra work
    let byteidx = if nibidx & 1 == 0 {
        byteidx
    // shifting odd nibbles requires replacing lo without removing hi
    } else {
        *slice[byteidx].byte_mut() &= 0xF0;
        *slice[byteidx].byte_mut() |= slice[byteidx + 1].hi().to_lo();
        byteidx + 1
    };

    for i in byteidx..(bytelen - 1) {
        *slice[i].byte_mut() <<= 4;
        *slice[i].byte_mut() |= slice[i + 1].hi().to_lo();
    }

    // the last byte has nothing to shift left
    *slice[bytelen - 1].byte_mut() <<= 4;
}

pub(crate) fn set_nib<T: u4>(slice: &mut [u4x2], nibidx: usize, nib: T) {
    let idx = nibidx >> 1;
    if nibidx & 1 == 0 {
        slice[idx].set_hi(nib);
    } else {
        slice[idx].set_lo(nib);
    }
}

pub(crate) fn get_nib<T: u4>(slice: &[u4x2], nibidx: usize) -> T {
    let idx = nibidx >> 1;
    if nibidx & 1 == 0 {
        T::from_hi(slice[idx].hi().to_hi())
    } else {
        T::from_lo(slice[idx].lo().to_lo())
    }
}

pub(crate) fn get_nib_ref(slice: &[u4x2], nibidx: usize) -> &dyn u4 {
    let idx = nibidx >> 1;
    if nibidx & 1 == 0 {
        slice[idx].hi()
    } else {
        slice[idx].lo()
    }
}

pub(crate) fn get_nib_mut(slice: &mut [u4x2], nibidx: usize) -> &dyn U4Cell {
    let idx = nibidx >> 1;
    if nibidx & 1 == 0 {
        slice[idx].hi_mut()
    } else {
        slice[idx].lo_mut()
    }
}

pub(crate) trait ToLo {
    fn to_lo(&self) -> u8;
}
impl ToLo for u8 {
    fn to_lo(&self) -> u8 {
        *self
    }
}
impl ToLo for u16 {
    fn to_lo(&self) -> u8 {
        *self as u8
    }
}
impl ToLo for u32 {
    fn to_lo(&self) -> u8 {
        *self as u8
    }
}
impl ToLo for u64 {
    fn to_lo(&self) -> u8 {
        *self as u8
    }
}
impl ToLo for usize {
    fn to_lo(&self) -> u8 {
        *self as u8
    }
}
