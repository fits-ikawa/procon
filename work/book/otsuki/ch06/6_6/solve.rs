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
        n: usize, k: i32,
        a: [i32; n],
        mut b: [i32; n],
    }

    b.sort();

    let ans = a
        .into_iter()
        .filter_map(|ai| match b.lower_bound(&(k - ai)) {
            j if j < n => Some(ai + b[j]),
            _ => None,
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
