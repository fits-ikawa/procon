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
        n: usize, k: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort();
    b.sort();

    let mut left = 0;
    let mut right = a.last().unwrap() * b.last().unwrap();

    while right - left > 1 {
        let mid = (left + right) / 2;

        let count: usize = a
            .iter()
            .map(|&ai| b.upper_bound_by(|&bi| (ai * bi).cmp(&mid)))
            .sum();

        if count >= k {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
