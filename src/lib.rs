//! This crate contains all sorts of types for dealing with nibbles, i.e. four-byte numbers.
//! Curretly, only unsigned nibbles are supported.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![doc(html_root_url = "https://docs.charr.xyz/nibble/")]
#![feature(try_from)]
#![cfg_attr(test, deny(missing_debug_implementations, missing_docs, warnings))]
#![feature(trace_macros)]

extern crate arrayvec;
extern crate core;
extern crate num_traits;

mod cmp;
mod common;
mod fmt;
mod num;
mod ops;
pub mod array;
pub mod base;
pub mod iter;
pub mod pair;
pub mod slice;
#[cfg(feature = "std")]
pub mod vec;
pub use array::{NibArrayVec, NibArray};
pub use base::{u4, u4hi, u4lo};
pub use pair::{u4x2, U4Cell, U4LoCell, U4HiCell};
pub use slice::{NibSlice, NibSliceMut, NibSliceExt, NibSliceMutExt};
#[cfg(feature = "std")]
pub use vec::{NibVec};
