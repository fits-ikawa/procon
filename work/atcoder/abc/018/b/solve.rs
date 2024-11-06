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
        mut s: Chars,
        n: usize,
        lrs: [(usize, usize); n],
    }

    for (l, r) in lrs {
        let (l, r) = (l - 1, r - 1);
        for i in 0..=(r - l) / 2 {
            s.swap(l + i, r - i);
        }
    }

    println!("{}", s.iter().join(""));
}
