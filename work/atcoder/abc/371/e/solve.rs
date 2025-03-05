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
    // 解説 AC
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut idx = vec![vec![0]; n];

    for i in 0..n {
        idx[a[i]].push(i + 1);
    }

    for i in 0..n {
        idx[i].push(n + 1);
    }

    let mut ans = 0;

    for i in 0..n {
        ans += n * (n + 1) / 2
            - idx[i]
                .iter()
                .tuple_windows()
                .map(|(&l, &r)| {
                    let m = r - l - 1;
                    m * (m + 1) / 2
                })
                .sum::<usize>();
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut acc = vec![0; n];
    let mut seen = vec![false; n];

    acc[0] = 1;
    seen[a[0]] = true;

    for i in 1..n {
        acc[i] = acc[i - 1] + if seen[a[i]] { 0 } else { 1 };
        seen[a[i]] = true;
    }

    let mut idx = vec![vec![]; n];

    for i in 0..n {
        idx[a[i]].push(i);
    }

    for i in 0..n {
        idx[i].push(n);
    }

    let mut sum = acc.iter().sum::<usize>();
    let mut ans = sum;

    for i in 0..n - 1 {
        let pos = idx[a[i]].upper_bound(&i);
        sum -= idx[a[i]][pos] - i;
        ans += sum;
    }

    println!("{}", ans);
}
