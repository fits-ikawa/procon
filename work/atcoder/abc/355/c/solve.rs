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
    // O(N+T) 解法
    input! {
        n: usize, t: usize,
        a: [usize; t],
    }

    let mut row = vec![0; n];
    let mut col = vec![0; n];
    let mut diag1 = 0;
    let mut diag2 = 0;

    for i in 0..t {
        let num = a[i] - 1;
        let x = num / n;
        let y = num % n;

        row[x] += 1;
        col[y] += 1;

        if x == y {
            diag1 += 1;
        }
        if x == n - 1 - y {
            diag2 += 1;
        };

        if row[x] == n || col[y] == n || diag1 == n || diag2 == n {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
fn solve() {
    // O(N^2 + T) 解法
    input! {
        n: usize, t: usize,
        a: [usize; t],
    }

    let mut sheet = vec![vec![None; n]; n];

    for i in 0..t {
        let num = a[i] - 1;
        let x = num / n;
        let y = num % n;
        sheet[x][y] = Some(i + 1);
    }

    let mut ans = usize::MAX;

    let mut check = |hit: &[usize]| {
        if hit.len() == n {
            ans = ans.min(hit.iter().max().copied().unwrap());
        }
    };

    for i in 0..n {
        // 横
        check(&(0..n).filter_map(|j| sheet[i][j]).collect_vec());

        // 縦
        check(&(0..n).filter_map(|j| sheet[j][i]).collect_vec());
    }

    // 斜め（左上から右下）
    check(&(0..n).filter_map(|i| sheet[i][i]).collect_vec());

    // 斜め（右上から左下）
    check(&(0..n).filter_map(|i| sheet[i][n - 1 - i]).collect_vec());

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
