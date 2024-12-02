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
    }

    // 重複組み合わせ (11, 3) = 286, (12, 3) = 364 なので
    // 12 桁のレピュニット数まであれば 333 番目までの答えを求められる
    // （サンプルの 3 番目からもわかる）
    let reps: [usize; 12] = [
        1,
        11,
        111,
        1111,
        11111,
        111111,
        1111111,
        11111111,
        111111111,
        1111111111,
        11111111111,
        111111111111,
    ];

    let mut sums = hashset! {};

    for (i, j, k) in iproduct!(0..12, 0..12, 0..12) {
        sums.insert(reps[i] + reps[j] + reps[k]);
    }

    let ans = sums.iter().sorted().nth(n - 1).unwrap();

    println!("{}", ans);
}
