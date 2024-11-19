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
        ab: [(f64, f64); n],
    }

    let mut total_time = 0.0;

    for &(a, b) in &ab {
        total_time += a / b;
    }

    let mut time = 0.0;
    let mut ans = 0.0;

    // 時間的に半分の点で B 側の火と出会う
    for &(a, b) in &ab {
        if time + a / b <= total_time / 2.0 {
            time += a / b;
            ans += a;
        } else {
            ans += a * (total_time / 2.0 - time) / (a / b);
            break;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // 二分探索でゴリ押し
    input! {
        n: usize,
        ab: [(f64, f64); n],
    }

    let len = ab.iter().map(|(a, _)| a).sum::<f64>();

    let mut left = 0.0;
    let mut right = len;

    let mut ans = 0.0;

    while right - left > 1e-8 {
        let mid = (left + right) / 2.0;

        let mut len_l = 0.0;
        let mut time_l = 0.0;
        for &(a, b) in &ab {
            if len_l + a <= mid {
                time_l += a / b;
                len_l += a;
            } else {
                time_l += (mid - len_l) / b;
                len_l = mid;
            }
        }

        let mut len_r = 0.0;
        let mut time_r = 0.0;
        for &(a, b) in ab.iter().rev() {
            if len_r + a <= len - mid {
                time_r += a / b;
                len_r += a;
            } else {
                time_r += (len - mid - len_r) / b;
                len_r = len - mid;
            }
        }

        ans = len_l;

        if time_l >= time_r {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", ans);
}
