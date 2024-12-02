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
        mut fs: [(usize, usize); n],
    }

    fs.sort_by_key(|&(_, s)| s);
    fs.reverse();

    let mut ans = 0;

    for i in 1..n {
        ans = ans.max(if fs[0].0 == fs[i].0 {
            fs[0].1 + fs[i].1 / 2
        } else {
            fs[0].1 + fs[i].1
        });
    }

    println!("{}", ans);
}
