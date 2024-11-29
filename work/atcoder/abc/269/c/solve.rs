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

    let bits = (0..usize::BITS - n.leading_zeros())
        .filter(|&i| n & (1 << i) > 0)
        .collect_vec();

    let mut ans = vec![];

    for k in 0..=bits.len() {
        for c in bits.iter().combinations(k) {
            let x = c.into_iter().map(|&i| 1 << i).sum::<usize>();
            ans.push(x);
        }
    }

    ans.sort();

    println!("{}", ans.iter().join("\n"));
}
