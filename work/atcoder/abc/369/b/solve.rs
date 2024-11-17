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
    }

    let mut left = vec![];
    let mut right = vec![];

    for _ in 0..n {
        input! {
            a: usize, s: char,
        }
        if s == 'L' {
            left.push(a);
        } else {
            right.push(a);
        }
    }

    let ans_left = left
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<usize>();

    let ans_right = right
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<usize>();

    println!("{}", ans_left + ans_right);
}
