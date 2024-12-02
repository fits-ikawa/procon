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
    // クエリ部分 O(N) （解説 AC）
    // どのみちソートしてるので全体計算量は N(NlogN) で変わらず。ちょっとだけ速い
    input! {
        n: usize,
        a: [usize; n],
    }

    let a_sorted = a.iter().sorted().rev().copied().collect_vec();

    let mut table = hashmap! {};
    let mut sum = a_sorted[0];
    table.insert(a_sorted[0], 0);

    for i in 1..n {
        if a_sorted[i - 1] != a_sorted[i] {
            table.insert(a_sorted[i], sum);
        }
        sum += a_sorted[i];
    }

    let mut ans = vec![];

    for ai in a {
        ans.push(table[&ai]);
    }

    println!("{}", ans.iter().join(" "));
}

#[allow(dead_code)]
fn solve() {
    // クエリ部分 O(NlogN)
    input! {
        n: usize,
        a: [usize; n],
    }

    let a_sorted = a.iter().sorted().copied().collect_vec();

    let acc = std::iter::once(0)
        .chain(a_sorted.iter().copied())
        .cumsum::<usize>()
        .collect_vec();

    let mut ans = vec![];

    for ai in a {
        let pos = a_sorted.upper_bound(&ai);
        ans.push(acc[n] - acc[pos]);
    }

    println!("{}", ans.iter().join(" "));
}
