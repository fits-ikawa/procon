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
        n: usize, d: usize, p: usize,
        mut f: [usize; n],
    }

    f.sort();
    let acc = std::iter::once(0).chain(f).cumsum::<usize>().collect_vec();

    let mut ans = usize::MAX;

    // div_ceil が AtCoder 環境だとエラー 2024/11/30
    for i in 0..=n.div_ceil(d) {
        let cost = i * p + acc[n.saturating_sub(d * i)];
        ans = ans.min(cost);
    }

    println!("{}", ans);
}
