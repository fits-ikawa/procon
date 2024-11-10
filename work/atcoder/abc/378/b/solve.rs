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
        qr: [(isize, isize); n],
        q: usize,
        td: [(Usize1, isize); q],
    }

    for (t, d) in td {
        let (q, r) = qr[t];
        let check = r - d % q;
        let ans = match check.cmp(&0) {
            Equal | Greater => d + check,
            Less => d + q + check,
        };
        println!("{}", ans);
    }
}
