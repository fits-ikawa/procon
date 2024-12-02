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
        n: usize, _m: usize,
        s: [String; n],
    }

    for p in s.iter().permutations(n) {
        if p.windows(2).all(|w| {
            let (a, b) = (w[0], w[1]);
            izip!(a.chars(), b.chars()).filter(|&(a, b)| a != b).count() == 1
        }) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
