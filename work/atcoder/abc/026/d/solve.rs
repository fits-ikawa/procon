#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*, f64::consts::PI};
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
        a: f64, b: f64, c: f64,
    }

    let f = |t: f64| a * t + b * f64::sin(c * t * PI) - 100.0;

    let mut left = 0.0;
    let mut right = 100.0;

    while f(right).abs() > 1e-6 {
        let mid = (left + right) / 2.0;
        debug!(left, right, mid, f(mid));

        if f(mid) >= 0.0 {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
