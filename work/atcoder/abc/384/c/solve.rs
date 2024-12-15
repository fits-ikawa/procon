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
        a: usize, b: usize, c: usize, d: usize, e: usize,
    }

    let c2s = hashmap! {
        'A' => a,
        'B' => b,
        'C' => c,
        'D' => d,
        'E' => e,
    };

    let mut names = vec![];

    for c in "ABCDE".chars().powerset() {
        if !c.is_empty() {
            names.push(c);
        }
    }

    let scores = names
        .iter()
        .map(|name| {
            (
                Reverse(name.iter().map(|x| c2s[x]).sum::<usize>()),
                name.iter().collect::<String>(),
            )
        })
        .sorted()
        .collect_vec();

    println!("{}", scores.iter().map(|(_, n)| n).join("\n"));
}
