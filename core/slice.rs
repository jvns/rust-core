// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::mem::{nonzero_size_of, size_of, transmute};
use super::ptr::{offset, read_ptr, swap_ptr};
use super::fail::abort;
use super::container::Container;
use super::option::{Option, Some, None};
use super::clone::Clone;
use super::iter::Iterator;

pub struct Slice<T> {
    data: *T,
    len: uint
}

pub unsafe fn unchecked_get<'a, T>(xs: &'a [T], index: uint) -> &'a T {
    let slice: Slice<T> = transmute(xs);
    transmute(offset(slice.data, index as int))
}

pub unsafe fn unchecked_mut_get<'a, T>(xs: &'a mut [T], index: uint) -> &'a mut T {
    let slice: Slice<T> = transmute(xs);
    transmute(offset(slice.data, index as int))
}

pub fn to_ptr<T>(xs: &[T]) -> *T {
    unsafe {
        let slice: Slice<T> = transmute(xs);
        slice.data
    }
}

pub fn slice<'a, T>(xs: &'a [T], start: uint, end: uint) -> &'a [T] {
    if start > end || end > xs.len() {
        abort()
    }
    unsafe {
        let slice: Slice<T> = transmute(xs);
        let new = Slice {
            data: offset(slice.data, start as int),
            len: (end - start)
        };
        transmute(new)
    }
}

pub fn slice_from<'a, T>(xs: &'a [T], start: uint) -> &'a [T] {
    slice(xs, start, xs.len())
}

pub fn slice_to<'a, T>(xs: &'a [T], end: uint) -> &'a [T] {
    slice(xs, 0, end)
}

pub fn split<'a, T>(xs: &'a [T], mid: uint) -> (&'a [T], &'a [T]) {
    (slice_to(xs, mid), slice_from(xs, mid))
}

pub fn to_mut_ptr<T>(xs: &mut [T]) -> *mut T {
    unsafe {
        let slice: Slice<T> = transmute(xs);
        slice.data as *mut T
    }
}

pub fn mut_slice<'a, T>(xs: &'a mut [T], start: uint, end: uint) -> &'a mut [T] {
    if start > end || end > xs.len() {
        abort()
    }
    unsafe {
        let slice: Slice<T> = transmute(xs);
        let new = Slice {
            data: offset(slice.data, start as int),
            len: (end - start)
        };
        transmute(new)
    }
}

pub fn mut_slice_from<'a, T>(xs: &'a mut [T], start: uint) -> &'a mut [T] {
    let length = xs.len();
    mut_slice(xs, start, length)
}

pub fn mut_slice_to<'a, T>(xs: &'a mut [T], end: uint) -> &'a mut [T] {
    mut_slice(xs, 0, end)
}

pub fn mut_split<'a, T>(xs: &'a mut [T], mid: uint) -> (&'a mut [T], &'a mut [T]) {
    unsafe {
        let ys: &'a mut [T] = read_ptr(&xs);
        (mut_slice_to(xs, mid), mut_slice_from(ys, mid))
    }
}

pub fn swap<T>(xs: &mut [T], a: uint, b: uint) {
    unsafe {
        let x: *mut T = &mut xs[a];
        let y: *mut T = &mut xs[b];
        swap_ptr(x, y);
    }
}

pub unsafe fn unchecked_swap<T>(xs: &mut [T], a: uint, b: uint) {
    let x: *mut T = unchecked_mut_get(xs, a);
    let y: *mut T = unchecked_mut_get(xs, b);
    swap_ptr(x, y);
}


impl<'a, T> Container for &'a [T] {
    fn len(&self) -> uint {
        unsafe {
            let slice: Slice<T> = transmute(*self);
            slice.len
        }
    }
}

pub fn iter<'a, T>(xs: &'a [T]) -> VecIterator<'a, T> {
    unsafe {
        let p = to_ptr(xs);
        if size_of::<T>() == 0 {
            VecIterator { ptr: p, end: (p as uint + xs.len()) as *T, lifetime: None }
        } else {
            VecIterator { ptr: p, end: offset(p, xs.len() as int), lifetime: None}
        }
    }
}

pub fn mut_iter<'a, T>(xs: &'a mut [T]) -> VecMutIterator<'a, T> {
    unsafe {
        let p = to_mut_ptr(xs);
        if size_of::<T>() == 0 {
            VecMutIterator{ptr: p, end: (p as uint + xs.len()) as *mut T, lifetime: None}
        } else {
            VecMutIterator{ptr: p, end: offset(p as *T, xs.len() as int) as *mut T, lifetime: None}
        }
    }
}

macro_rules! iterator {
    (struct $name:ident -> $ptr:ty, $elem:ty) => {
        /// An iterator for iterating over a slice.
        pub struct $name<'self, T> {
            priv ptr: $ptr,
            priv end: $ptr,
            priv lifetime: Option<$elem> // https://github.com/mozilla/rust/issues/5922
        }

        impl<'self, T> Iterator<$elem> for $name<'self, T> {
            #[inline]
            fn next(&mut self) -> Option<$elem> {
                // could be implemented with slices, but this avoids bounds checks
                unsafe {
                    if self.ptr == self.end {
                        None
                    } else {
                        let old = self.ptr;
                        self.ptr = if size_of::<T>() == 0 {
                            // `offset` will return the same pointer for 0-size types
                            transmute(self.ptr as uint + 1)
                        } else {
                            offset(self.ptr as *T, 1) as $ptr
                        };

                        Some(transmute(old))
                    }
                }
            }

            #[inline]
            fn size_hint(&self) -> (uint, Option<uint>) {
                let diff = (self.end as uint) - (self.ptr as uint);
                let exact = diff / nonzero_size_of::<T>();
                (exact, Some(exact))
            }
        }
    }
}

iterator!{struct VecIterator -> *T, &'self T}
iterator!{struct VecMutIterator -> *mut T, &'self mut T}

impl<'a, T> Clone for VecIterator<'a, T> {
    fn clone(&self) -> VecIterator<'a, T> {
        *self
    }
}
