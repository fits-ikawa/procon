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
    // 解析的に解く
    input! {
        a: f64, b: f64,
    }

    let c = 2.0 * (b / a);
    let mut g = 1.0 / (c * c).cbrt();

    g = if g < 1.0 {
        1.0
    } else if f(g.floor(), a, b) < f(g.ceil(), a, b) {
        g.floor()
    } else {
        g.ceil()
    };

    println!("{}", f(g, a, b));
}

fn f(g: f64, a: f64, b: f64) -> f64 {
    a / g.sqrt() + b * (g - 1.0)
}

#[allow(dead_code)]
fn solve() {
    // 三分探索で解く
    input! {
        a: f64, b: f64,
    }

    let mut left = 0.0;
    let mut right = 1e18;

    while right - left > 1.0 {
        let mid1 = (left * 2.0 + right) / 3.0;
        let mid2 = (left + right * 2.0) / 3.0;

        if f(mid1, a, b) < f(mid2, a, b) {
            right = mid2;
        } else {
            left = mid1;
        }
    }

    let mut ans = f64::MAX;

    for i in (left.floor() as usize).max(1)..=(right.ceil() as usize).max(1) {
        ans = ans.min(f(i as f64, a, b));
    }

    println!("{}", ans);
}
