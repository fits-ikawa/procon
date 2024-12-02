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
        n: usize, m: usize,
    }

    let mut ans = vec![];

    dfs(1, 1, &mut vec![], &mut ans, n, m);

    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter().map(|line| line.iter().join(" ")).join("\n")
    );
}

fn dfs(cur: usize, i: usize, seq: &mut Vec<usize>, ans: &mut Vec<Vec<usize>>, n: usize, m: usize) {
    if seq.len() == n {
        ans.push(seq.clone());
        return;
    }

    for j in cur..=(m - (n - i) * 10) {
        seq.push(j);
        dfs(j + 10, i + 1, seq, ans, n, m);
        seq.pop();
    }
}
