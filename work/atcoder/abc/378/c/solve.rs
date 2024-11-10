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
        a: [isize; n],
    }

    let mut table = hashmap! {};
    let mut b = vec![];

    for (i, ai) in a.into_iter().enumerate() {
        b.push(table.get(&ai).copied().unwrap_or(-1));
        table.insert(ai, (i + 1) as isize);
    }

    println!("{}", b.iter().join(" "));
}
