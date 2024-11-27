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

    let ranges = (0..n)
        .map(|_| {
            input! {
                t: usize, l:usize, r:usize,
            }

            match t {
                1 => (l * 2, r * 2 + 1),
                2 => (l * 2, r * 2),
                3 => (l * 2 + 1, r * 2 + 1),
                4 => (l * 2 + 1, r * 2),
                _ => unreachable!(),
            }
        })
        .collect_vec();

    let mut ans = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            let (a, b) = ranges[i];
            let (c, d) = ranges[j];

            if a.max(c) < b.min(d) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // float で解く（入力値の制約によっては誤差が問題になるかも）
    input! {
        n: usize,
    }

    let ranges = (0..n)
        .map(|_| {
            input! {
                t: usize, l:f64, r:f64,
            }

            match t {
                1 => (l, r),
                2 => (l, r - 0.1),
                3 => (l + 0.1, r),
                4 => (l + 0.1, r - 0.1),
                _ => unreachable!(),
            }
        })
        .collect_vec();

    let mut ans = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            let a = ranges[i];
            let b = ranges[j];

            if a.0 <= b.1 && b.0 <= a.1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
