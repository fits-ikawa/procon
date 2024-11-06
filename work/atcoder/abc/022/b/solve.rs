#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
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
        fs: [usize; n],
    }

    let count = fs
        .into_iter()
        .fold((0, hashset! {}), |(acc, mut set), f| {
            if set.contains(&f) {
                (acc + 1, set)
            } else {
                set.insert(f);
                (acc, set)
            }
        })
        .0;

    println!("{}", count);
}
