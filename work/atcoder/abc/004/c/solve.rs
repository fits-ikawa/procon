#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

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

    let mut cards = [1, 2, 3, 4, 5, 6];

    let n = n % 30; // 30 回で最初の状態に戻る

    for i in 0..n {
        cards.swap(i % 5, i % 5 + 1);
    }

    println!("{}", cards.iter().join(""));
}
