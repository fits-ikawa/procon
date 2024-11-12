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
        mut m: usize,
    }

    let mut ans = vec![];

    for i in (0..=10).rev() {
        let a = 3_usize.pow(i);
        for _ in 0..m / a {
            ans.push(i);
        }
        m %= 3_usize.pow(i);
    }

    // debug!(ans.iter().map(|&x| 3_usize.pow(x)).sum::<usize>());

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
