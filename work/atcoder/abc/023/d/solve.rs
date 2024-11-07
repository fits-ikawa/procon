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
        n: usize,
        baloons: [(usize, usize); n],
    }

    let mut left = 0;
    let mut right = baloons.iter().map(|&(h, s)| h + s * (n - 1)).max().unwrap();

    'outer: while right - left > 1 {
        let mid = (left + right) / 2;

        let mut limits = vec![0; n];
        for i in 0..n {
            let (h, s) = baloons[i];
            if h > mid {
                left = mid;
                continue 'outer;
            }
            limits[i] = (mid - h) / s;
        }
        limits.sort_unstable();

        if limits.into_iter().enumerate().all(|(i, v)| v >= i) {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        baloons: [(i64, i64); n],
    }

    let mut left: i64 = 0;
    let mut right = baloons
        .iter()
        .map(|&(h, s)| h + s * (n - 1) as i64)
        .max()
        .unwrap();

    while right - left > 1 {
        let mid = (left + right) / 2;

        let ok = baloons
            .iter()
            .map(|&(h, s)| if h > mid { -1 } else { (mid - h) / s })
            .sorted()
            .enumerate()
            .all(|(i, v)| i as i64 <= v);

        if ok {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
