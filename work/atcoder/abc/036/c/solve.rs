#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use proconio::{marker::*, *};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let rank = a
        .iter()
        .unique()
        .sorted()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    let ans = a.iter().map(|ai| rank[ai]).join("\n");

    println!("{}", ans);
}
