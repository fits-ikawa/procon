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
        x: i128, a: i128, d:i128, n:i128,
    }

    let min = a.min(a + (d * (n - 1)));
    let max = a.max(a + (d * (n - 1)));

    if x <= min {
        println!("{}", min - x);
        return;
    } else if x >= max {
        println!("{}", x - max);
        return;
    }

    let pos = (x - a).abs() % d.abs();

    println!("{}", pos.min(d.abs() - pos));
}
