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
        x1: isize, y1: isize, x2:isize, y2:isize,
    }

    let mut check = hashset! {};

    for &(dx, dy) in &IDIRK {
        check.insert((x1 + dx, y1 + dy));
        check.insert((x2 + dx, y2 + dy));
    }

    println!("{}", if check.len() < 16 { "Yes" } else { "No" });
}

// 桂馬の移動リスト
const IDIRK: [(isize, isize); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];
