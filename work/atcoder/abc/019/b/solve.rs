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
        s: Chars,
    }

    let compressed = s
        .into_iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(c, group)| format!("{}{}", c, group.count()))
        .join("");

    println!("{}", compressed);
}
