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
        a: [Usize1; n],
    }

    let good = a.iter().enumerate().filter(|&(i, &ai)| ai == i).count();

    let mut ans = good * (good - 1) / 2;

    for (i, &ai) in a.iter().enumerate() {
        if ai > i && a[ai] == i {
            ans += 1;
        }
    }

    println!("{}", ans);
}
