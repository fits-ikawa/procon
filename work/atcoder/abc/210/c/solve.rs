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
        c: [usize; n],
    }

    let mut count = c[0..k].iter().copied().counts();
    let mut kind = c[0..k].iter().unique().count();
    let mut ans = kind;

    for i in 1..n - k + 1 {
        let value = count.get_mut(&c[i - 1]).unwrap();
        *value -= 1;
        if *value == 0 {
            kind -= 1;
        }

        let value = count.entry(c[i + k - 1]).or_insert(0);
        *value += 1;
        if *value == 1 {
            kind += 1;
        }

        ans = ans.max(kind);
    }

    println!("{}", ans);
}
