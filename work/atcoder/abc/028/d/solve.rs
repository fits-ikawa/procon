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
        n: f64, k: f64,
    }

    // (K より小さい数, K, K より大きい数) * 並べ替え 6 通り +
    // (K, K, K 以外) * 並べ替え 3 通り +
    // (K, K, K) * 並べ替え 1 通り
    let m = (k - 1.0) * (n - k) * 6.0 + (n - 1.0) * 3.0 + 1.0;

    println!("{}", m / n.powi(3));
}
