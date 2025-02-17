#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cell::RefCell;
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        c: [usize; n],
        ab: [(Usize1, Usize1); q],
    }

    let boxes = vec![RefCell::new(hashset! {}); n];

    for i in 0..n {
        boxes[i].borrow_mut().insert(c[i]);
    }

    for (a, b) in ab {
        let mut ba = boxes[a].take();
        let mut bb = boxes[b].take();

        if ba.len() >= bb.len() {
            ba.extend(bb);
            boxes[b].replace(ba);
        } else {
            bb.extend(ba);
            boxes[b].replace(bb);
        }

        println!("{}", boxes[b].borrow().len());
    }
}
