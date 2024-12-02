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
        mut a: [usize; n],
    }

    let sum = a.iter().sum::<usize>();
    let average = sum / n;

    a.sort();
    a.reverse();

    let ans = a
        .iter()
        .enumerate()
        .map(|(i, &ai)| ai.abs_diff(average + if i < sum % n { 1 } else { 0 }))
        .sum::<usize>()
        / 2;

    println!("{}", ans);
}
