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
        _n: usize,
        a: usize, b: usize,
        k: usize,
        mut path: [usize; k],
    }

    path.push(a);
    path.push(b);
    let actual_len = path.len();

    path.sort_unstable();
    path.dedup();
    let shortest_len = path.len();

    println!(
        "{}",
        if actual_len == shortest_len {
            "YES"
        } else {
            "NO"
        }
    );
}
