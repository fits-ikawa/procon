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
        n: usize, a: usize, b:usize,
    }

    let ans = (0..a * n)
        .map(|x| {
            (0..b * n)
                .map(|y| if (x / a + y / b) % 2 == 0 { '.' } else { '#' })
                .collect::<String>()
        })
        .join("\n");

    println!("{}", ans);
}
