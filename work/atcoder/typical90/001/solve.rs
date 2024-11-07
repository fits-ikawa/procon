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
        n: usize, l: usize,
        k: usize,
        a: [usize; n],
    }

    let mut left = 0;
    let mut right = l;

    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut count = 0;
        let mut prev = 0;
        for &ai in &a {
            if ai - prev >= mid {
                count += 1;
                prev = ai;
            }
        }
        if l - prev >= mid {
            count += 1;
        }

        if count > k {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
