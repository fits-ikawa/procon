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
        n: usize, t: usize,
        a: [usize; n],
    }

    let ans = a.windows(2).map(|x| (x[1] - x[0]).min(t)).sum::<usize>() + t;

    println!("{}", ans);
}
