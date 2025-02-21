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

    let mut boxes = vec![hashset! {}; n];

    for i in 0..n {
        boxes[i].insert(c[i]);
    }

    for (a, b) in ab {
        let mut ba = std::mem::take(&mut boxes[a]);
        let mut bb = std::mem::take(&mut boxes[b]);

        if ba.len() >= bb.len() {
            ba.extend(bb);
            boxes[b] = ba;
        } else {
            bb.extend(ba);
            boxes[b] = bb;
        }

        println!("{}", boxes[b].len());
    }
}
