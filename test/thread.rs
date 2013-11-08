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

use core::thread::spawn;
use core::fail::abort;

#[path = "../core/mod.rs"]
mod core;

static mut a: bool = false;
static mut b: bool = false;

fn foo() {
    unsafe { a = true };
}

fn bar() {
    unsafe { b = true };
}

#[start]
fn main(_: int, _: **u8) -> int {
    {
        let _a = spawn(foo);
        let _b = spawn(bar);
    }
    unsafe {
        if !a || !b {
            abort()
        }
    }
    0
}
