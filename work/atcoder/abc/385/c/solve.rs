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
    // 解説 AC
    // 工夫なしの全探索でよかった
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = 0;

    for i in 0..n {
        for j in 1..=n {
            ans = ans.max(
                h[i..]
                    .iter()
                    .step_by(j)
                    .take_while(|&&hk| hk == h[i])
                    .count(),
            );
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // コンテスト中 AC
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut idx = hashmap! {};
    let mut idx_table = vec![vec![false; 3000]; 3001];

    for (i, hi) in h.into_iter().enumerate() {
        let value = idx.entry(hi).or_insert(vec![]);
        (*value).push(i);
        idx_table[hi][i] = true;
    }

    let mut ans = 0;

    for (height, index) in idx {
        if index.len() == 1 {
            ans = ans.max(1);
            continue;
        }

        for i in 0..index.len() - 1 {
            for j in i + 1..index.len() {
                let mut cnt = 2;
                let d = index[j] - index[i];
                let mut next = index[j] + d;
                while next < 3000 && idx_table[height][next] {
                    cnt += 1;
                    next += d;
                }
                ans = ans.max(cnt);
            }
        }
    }

    println!("{}", ans);
}
