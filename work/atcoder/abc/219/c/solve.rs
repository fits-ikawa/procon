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
        x: Chars,
        n: usize,
        s: [String; n],
    }

    let table = ('a'..='z')
        .enumerate()
        .map(|(i, c)| (x[i], c))
        .collect::<HashMap<_, _>>();

    let t = s
        .iter()
        .map(|si| (si.chars().map(|c| table[&c]).collect::<String>(), si))
        .sorted()
        .collect_vec();

    println!("{}", t.iter().map(|(_, s)| s).join("\n"));
}
