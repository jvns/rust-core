// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern "rust-intrinsic" {
    fn u64_add_with_overflow(x: u64, y: u64) -> (u64, bool);
    fn u64_sub_with_overflow(x: u64, y: u64) -> (u64, bool);
    fn u64_mul_with_overflow(x: u64, y: u64) -> (u64, bool);
}

#[inline(always)]
pub fn add_with_overflow(x: u64, y: u64) -> (u64, bool) {
    unsafe { u64_add_with_overflow(x, y) }
}

#[inline(always)]
pub fn sub_with_overflow(x: u64, y: u64) -> (u64, bool) {
    unsafe { u64_sub_with_overflow(x, y) }
}

#[inline(always)]
pub fn mul_with_overflow(x: u64, y: u64) -> (u64, bool) {
    unsafe { u64_mul_with_overflow(x, y) }
}

pub fn bswap(x: u64) -> u64 {
    super::i64::bswap(x as i64) as u64
}

#[cfg(target_endian = "big")]
pub fn to_be(x: u64) -> u64 {
    x
}

#[cfg(target_endian = "little")]
pub fn to_be(x: u64) -> u64 {
    bswap(x)
}

#[cfg(target_endian = "big")]
pub fn to_le(x: u64) -> u64 {
    bswap(x)
}

#[cfg(target_endian = "little")]
pub fn to_le(x: u64) -> u64 {
    x
}
