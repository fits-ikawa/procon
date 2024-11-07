#![allow(unused_imports)]
use itertools::*;
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
    }

    let abc = vec!['a', 'b', 'c'];

    let mut passwords: Vec<_> = vec![&abc; n]
        .into_iter()
        .multi_cartesian_product()
        .map(|chars| chars.iter().join(""))
        .collect();

    passwords.sort();

    println!("{}", passwords.iter().join("\n"));
}
