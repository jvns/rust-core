// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// AMD64 Linux is a LP64 platform.

pub type c_short     = i16;
pub type c_ushort    = u16;

pub type c_int       = i32;
pub type c_uint      = u32;

pub type c_long      = i64;
pub type c_ulong     = u64;

pub type c_longlong  = i64;
pub type c_ulonglong = u64;

pub type ssize_t     = i64;
pub type size_t      = u64;

pub struct pthread_t {
    priv size: c_ulong
}

pub struct pthread_attr_t {
    priv size: [u64, ..7]
}

pub struct pthread_mutex_t {
    priv size: [u64, ..5]
}

pub struct pthread_mutex_attr_t {
    priv size: u32
}

pub struct pthread_cond_t {
    priv size: [u64, ..6]
}

pub struct pthread_cond_attr_t {
    priv size: u32
}
