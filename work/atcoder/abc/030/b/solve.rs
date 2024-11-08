#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
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
        h: f64, m: f64,
    }

    let deg_h = (h % 12.0) * 30.0 + m / 2.0;
    let deg_m = m * 6.0;

    let deg_diff = (deg_h - deg_m).abs();

    println!("{}", deg_diff.min(360.0 - deg_diff));
}
