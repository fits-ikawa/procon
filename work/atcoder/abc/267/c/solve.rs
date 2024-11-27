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
        n: usize, m: usize,
        a: [isize; n],
    }

    let mut acc = a.iter().cumsum::<isize>().collect_vec();
    acc.insert(0, 0);

    let mut sum = a[..m]
        .iter()
        .enumerate()
        .map(|(i, ai)| (i as isize + 1) * ai)
        .sum::<isize>();

    let mut ans = sum;

    for i in 1..n - m + 1 {
        sum -= acc[i + m - 1] - acc[i - 1];
        sum += a[i + m - 1] * m as isize;
        ans = ans.max(sum);
    }

    println!("{}", ans);
}
