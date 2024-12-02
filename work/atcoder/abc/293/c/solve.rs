#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use superslice::*;

use counter::Counter;

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
        h: usize, w: usize,
        a: [[usize; w]; h],
    }

    let mut nums = Counter::new();
    nums.update([a[0][0]]);

    let ans = dfs(0, 0, &mut nums, h, w, &a);

    println!("{}", ans);
}

fn dfs(
    i: usize,
    j: usize,
    nums: &mut Counter<usize>,
    h: usize,
    w: usize,
    a: &[Vec<usize>],
) -> usize {
    if i == h - 1 && j == w - 1 {
        return if nums.len() == h + w - 1 { 1 } else { 0 };
    }

    let mut count = 0;

    if i != h - 1 {
        nums.update([a[i + 1][j]]);
        count += dfs(i + 1, j, nums, h, w, a);
        nums.subtract([a[i + 1][j]]);
    }

    if j != w - 1 {
        nums.update([a[i][j + 1]]);
        count += dfs(i, j + 1, nums, h, w, a);
        nums.subtract([a[i][j + 1]]);
    }

    count
}
