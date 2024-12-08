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
        s: Chars,
    }

    let mut left = vec![];
    let mut right = vec![];

    for i in 0..n {
        if s[i] == 'L' {
            right.push(i);
        } else {
            left.push(i);
        }
    }
    left.push(n);

    println!("{}", left.iter().chain(right.iter().rev()).join(" "));
}
