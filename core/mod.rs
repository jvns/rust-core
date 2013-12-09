// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[no_std];
#[allow(ctypes)];
#[link(name = "core")];
#[allow(attribute_usage)];
#[feature(macro_rules)];
#[macro_escape];

#[cfg(pthread)]
pub mod arc;
pub mod rc;
#[cfg(libc)]
pub mod weak;

pub mod atomic;
#[cfg(libc)]
pub mod deque;
pub mod char;
pub mod clone;
pub mod cmp;
#[cfg(libc, pthread)]
pub mod concurrent;
pub mod container;
pub mod fail;
#[cfg(libc)]
pub mod hash;
#[cfg(libc)]
pub mod io;
pub mod iter;
pub mod kinds;
#[cfg(libc)]
pub mod lru;
pub mod macros;
pub mod mem;
pub mod ops;
pub mod option;
#[cfg(libc)]
pub mod priority_queue;
pub mod ptr;
pub mod slice;
pub mod str;
#[cfg(libc, pthread)]
pub mod thread;
pub mod vec;

pub mod uint;
pub mod u8;
pub mod u16;
pub mod u32;
pub mod u64;

pub mod int;
pub mod i8;
pub mod i16;
pub mod i32;
pub mod i64;

#[cfg(target_os="linux", target_arch="x86")]
#[path="platform/linux-x86/c_types.rs"]
pub mod c_types;

#[cfg(target_os="linux", target_arch="x86_64")]
#[path="platform/linux-x86_64/c_types.rs"]
pub mod c_types;

#[cfg(libc)]
pub mod f32;
#[cfg(libc)]
pub mod f64;

#[cfg(libc)]
pub mod heap;

#[cfg(libc)]
pub mod heap_closure;

#[cfg(libc)]
pub mod os;

pub fn ignore<T>(_: T) {}
