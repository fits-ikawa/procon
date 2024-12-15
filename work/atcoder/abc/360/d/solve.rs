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
    // 尺取り法？で解く
    input! {
        n: usize, t: isize,
        s: Chars,
        x: [isize; n],
    }

    let mut to_l = vec![];
    let mut to_r = vec![];

    for i in 0..n {
        if s[i] == '0' {
            to_l.push(x[i]);
        } else {
            to_r.push(x[i]);
        }
    }

    to_l.sort();
    to_r.sort();

    let mut ans = 0;

    let mut pos_l = 0;
    let mut pos_r = 0;

    for i in 0..to_r.len() {
        while pos_l < to_l.len() && to_l[pos_l] <= to_r[i] {
            pos_l += 1;
        }

        while pos_r < to_l.len() && to_l[pos_r] <= to_r[i] + t * 2 {
            pos_r += 1;
        }

        ans += pos_r - pos_l;
    }

    println!("{}", ans);
}

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
fn solve() {
    // 二分探索で解く
    input! {
        n: usize, t: isize,
        s: Chars,
        x: [isize; n],
    }

    let mut to_l = vec![];
    let mut to_r = vec![];

    for i in 0..n {
        if s[i] == '0' {
            to_l.push(x[i]);
        } else {
            to_r.push(x[i]);
        }
    }

    to_l.sort();
    to_r.sort();

    let mut ans = 0;

    for i in 0..to_r.len() {
        let pos_l = to_l.upper_bound(&to_r[i]);
        let pos_r = to_l.upper_bound(&(to_r[i] + t * 2));
        ans += pos_r - pos_l;
    }

    println!("{}", ans);
}
