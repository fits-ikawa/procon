#![allow(clippy::comparison_chain)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        n: usize,
        mut v: [usize; n],
    }

    v.sort();
    v.reverse();

    let mut ans = v.pop().unwrap() as f64;

    while let Some(s) = v.pop() {
        ans = (ans + s as f64) / 2.0;
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        mut v: [usize; n],
    }

    v.sort();

    let mut a = vec![1];

    for i in 0..=n - 2 {
        a.push(2_usize.pow(i as u32));
    }

    let mut ans = 0;

    for (ai, vi) in izip!(a, v) {
        ans += ai * vi;
    }

    let ans = ans as f64 / 2_usize.pow((n - 1) as u32) as f64;

    println!("{}", ans);
}
