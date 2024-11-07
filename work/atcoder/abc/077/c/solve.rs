#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        b: [usize; n],
        mut c: [usize; n],
    }

    a.sort();
    c.sort();

    let ans: usize = b
        .into_iter()
        .map(|mid| a.lower_bound(&mid) * (n - c.upper_bound(&mid)))
        .sum();

    println!("{}", ans);
}
