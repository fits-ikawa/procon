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
        a: [usize; n],
    }

    let b = a.iter().sorted().dedup().copied().collect_vec();
    let mut k = vec![0; n];

    for i in 0..n {
        let pos = b.upper_bound(&a[i]);
        assert!(pos != 0);
        k[b.len() - pos] += 1;
    }

    println!("{}", k.iter().join("\n"));
}

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
fn solve() {
    // 解説 AC
    input! {
        n: usize,
        a: [usize; n],
    }

    let b = a
        .iter()
        .copied()
        .counts()
        .into_iter()
        .sorted_by_key(|&(k, _)| Reverse(k))
        .collect_vec();

    println!("{}", b.iter().map(|&(_, v)| v).join("\n"));

    for _ in 0..(n - b.len()) {
        println!("0");
    }
}
