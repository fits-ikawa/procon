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
        s: [String; n],
    }

    let ans = s
        .into_iter()
        .map(|si| {
            let rev = si.chars().rev().collect::<String>();
            if si <= rev {
                si
            } else {
                rev
            }
        })
        .sorted()
        .dedup()
        .count();

    println!("{}", ans);
}
