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
    // 尺取り法で解く
    input! {
        n: usize,
        a: [usize; n],
    }

    let size = a.iter().sum::<usize>();

    if size % 10 != 0 {
        println!("No");
        return;
    }

    let a2 = a.iter().chain(a.iter()).copied().collect_vec();

    let mut right = 0;
    let mut sum = a2[0];

    for left in 0..a2.len() {
        while right + 1 < a2.len() && sum + a2[right + 1] <= size / 10 {
            right += 1;
            sum += a2[right];
        }

        if sum == size / 10 {
            println!("Yes");
            return;
        }
        sum -= a2[left];

        if left == right && right + 1 < a2.len() {
            right += 1;
            sum = a2[right];
        }
    }

    println!("No");
}

#[allow(dead_code)]
fn solve() {
    // 二分探索で解く
    input! {
        n: usize,
        a: [usize; n],
    }

    let size = a.iter().sum::<usize>();

    if size % 10 != 0 {
        println!("No");
        return;
    }

    let mut acc = a.iter().chain(a.iter()).cumsum::<usize>().collect_vec();
    acc.insert(0, 0);

    for i in 0..acc.len() {
        if acc.binary_search(&(acc[i] + size / 10)).is_ok() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
