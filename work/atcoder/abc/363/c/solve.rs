#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        _n: usize, k: usize,
        mut s: Chars,
    }

    s.sort();
    let mut count = 0;

    loop {
        if s.windows(k)
            .all(|t| (0..t.len() / 2).any(|i| t[i] != t[t.len() - 1 - i]))
        {
            count += 1;
        }

        if !s.next_permutation() {
            break;
        }
    }

    println!("{}", count);
}

#[allow(dead_code)]
fn solve() {
    // ローカル環境だと 10 秒以上かかってしまう。
    // AtCoder のジャッジでは 2 秒ギリギリで AC できた。
    input! {
        n: usize, k: usize,
        s: Chars,
    }

    let ps: HashSet<_> = s.iter().permutations(n).collect();

    let count = ps
        .iter()
        .filter(|&ss| {
            ss.windows(k)
                .all(|t| (0..t.len() / 2).any(|i| t[i] != t[t.len() - 1 - i]))
        })
        .count();

    println!("{}", count);
}
