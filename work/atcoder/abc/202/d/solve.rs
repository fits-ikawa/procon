#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use memoise::memoise;
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
        mut a: usize, mut b: usize, mut k: usize,
    }

    let mut ans = vec![];

    for _ in 0..a + b {
        if a == 0 {
            ans.push('b');
            b -= 1;
        } else if b == 0 || comb(a - 1 + b, b) >= k {
            ans.push('a');
            a -= 1;
        } else {
            ans.push('b');
            k -= comb(a - 1 + b, b);
            b -= 1;
        }
    }

    println!("{}", ans.iter().collect::<String>());
}

#[memoise(x <= 60, y <= 30)]
fn comb(x: usize, y: usize) -> usize {
    assert!(x >= y);
    if y == 0 || x == y {
        return 1;
    }
    comb(x - 1, y - 1) + comb(x - 1, y)
}
