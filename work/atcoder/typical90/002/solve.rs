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
    }

    if n % 2 == 1 {
        return;
    }

    let mut ans = vec![];

    dfs(&mut vec![], 0, n, &mut ans);

    if !ans.is_empty() {
        println!("{}", ans.iter().join("\n"));
    }
}

fn dfs(seq: &mut Vec<char>, depth: usize, n: usize, ans: &mut Vec<String>) {
    if seq.len() == n {
        ans.push(seq.iter().collect::<String>());
        return;
    }

    if n - seq.len() > depth {
        seq.push('(');
        dfs(seq, depth + 1, n, ans);
        seq.pop();
    }

    if depth > 0 {
        seq.push(')');
        dfs(seq, depth - 1, n, ans);
        seq.pop();
    }
}
