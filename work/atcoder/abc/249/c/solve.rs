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
        n: usize, k: usize,
        s: [String; n],
    }

    let mut ans = 0;

    for i in k..=n {
        for c in s.iter().combinations(i) {
            let count = c
                .iter()
                .flat_map(|x| x.chars())
                .counts()
                .iter()
                .filter(|(_, &v)| v == k)
                .count();
            ans = ans.max(count);
        }
    }

    println!("{}", ans);
}
