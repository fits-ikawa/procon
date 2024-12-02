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
        k: usize,
    }

    let mut ans = vec![];

    for i in 0..=9 {
        dfs(i, 1, i, &mut ans);
    }

    ans.sort();

    // 0 も入っているのでそのまま k で参照する
    println!("{}", ans[k]);
}

fn dfs(cur: usize, wid: u32, seq: usize, ans: &mut Vec<usize>) {
    ans.push(seq);

    for i in cur + 1..=9 {
        let next = seq + i * 10_usize.pow(wid);
        dfs(i, wid + 1, next, ans);
    }
}
