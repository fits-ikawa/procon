#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering, Reverse};
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
fn main() {
    input! {
        n: usize, k: usize,
        mut s: Chars,
    }

    let mut ans = 0;

    for i in k..=n {
        if s[i - k..i] == vec!['O'; k] {
            ans += 1;
            for j in i - k..i {
                s[j] = 'X';
            }
        }
    }

    println!("{}", ans);
}
