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
        mut a: [usize; n],
        x: [usize; q],
    }

    a.sort();

    let acc = std::iter::once(0)
        .chain(a.iter().copied())
        .cumsum::<usize>()
        .collect_vec();

    for xi in x {
        let pos = a.upper_bound(&xi);
        let mut ans = xi * (pos) - (acc[pos] - acc[0]);
        ans += acc[n] - acc[pos] - xi * (n - pos);
        println!("{}", ans);
    }
}
