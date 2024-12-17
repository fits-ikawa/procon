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
        a: [isize; n],
        lrv: [(Usize1, Usize1, isize); q],
    }

    let mut d = a.windows(2).map(|w| w[1] - w[0]).collect_vec();
    let mut ans = d.iter().map(|x| x.abs()).sum::<isize>();

    for (l, r, v) in lrv {
        if l > 0 {
            ans -= d[l - 1].abs();
            d[l - 1] += v;
            ans += d[l - 1].abs();
        }
        if r < d.len() {
            ans -= d[r].abs();
            d[r] -= v;
            ans += d[r].abs();
        }
        println!("{}", ans);
    }
}
