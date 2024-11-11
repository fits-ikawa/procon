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
        n: usize, q: usize,
        x: [Usize1; q],
    }

    let mut i2num = (0..n).collect_vec();
    let mut num2i = (0..n).collect_vec();

    for xi in x {
        let i = num2i[xi];
        let j = if i == n - 1 { i - 1 } else { i + 1 };
        let xj = i2num[j];

        i2num[i] = xj;
        i2num[j] = xi;
        num2i[xi] = j;
        num2i[xj] = i;
    }

    println!("{}", i2num.iter().map(|&num| num + 1).join(" "));
}
