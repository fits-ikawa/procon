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
        n: usize, k: usize,
        mut videos: [usize; n],
    }

    videos.sort();
    let mut rate = 0.0;

    for &v in videos.iter().rev().take(k).rev() {
        rate = (rate + v as f64) / 2.0;
    }

    println!("{}", rate);
}
