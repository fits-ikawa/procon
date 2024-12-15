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
        s: Chars,
    }

    let mut indices = hashmap! {};

    for (i, si) in s.into_iter().enumerate() {
        (*indices.entry(si).or_insert(vec![])).push(i);
    }

    let mut ans = 0;

    for idx in indices.values() {
        let mut live = idx.len() - 1;

        for i in 1..idx.len() {
            ans += (idx[i] - idx[i - 1] - 1) * live;
            live -= i;
            ans += live;
            live += idx.len() - i - 1;
        }
    }

    println!("{}", ans);
}
