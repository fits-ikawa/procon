#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::collections::*;
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
    }

    let ans = (1..=n.sqrt())
        .map(|i| {
            let j = n / i;
            let area = i * j;
            let over = n - area;
            i.abs_diff(j) + over
        })
        .min()
        .unwrap_or(0);

    println!("{}", ans);
}
