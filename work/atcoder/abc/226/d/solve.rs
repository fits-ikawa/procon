#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut spells = hashset! {};

    for c in xy.iter().combinations(2) {
        let &(x1, y1) = c[0];
        let &(x2, y2) = c[1];

        let dx = x2 - x1;
        let dy = y2 - y1;

        let m = dx.gcd(&dy);

        spells.insert((dx / m, dy / m));
        spells.insert((-dx / m, -dy / m));
    }

    println!("{}", spells.len());
}
