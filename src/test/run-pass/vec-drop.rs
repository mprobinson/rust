// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::gc::{GC, Gc};

struct Pair { x: int, y: int }

pub fn main() {
    // This just tests whether the vec leaks its members.

    let _pvec: Vec<Gc<Pair>> =
        vec!(box(GC) Pair{x: 1, y: 2},
             box(GC) Pair{x: 3, y: 4},
             box(GC) Pair{x: 5, y: 6});
}
