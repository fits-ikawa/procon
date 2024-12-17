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
        h: usize, w: usize,
        a: [[usize; w]; h],
    }

    let mut row = vec![0; h];
    let mut col = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            row[i] += a[i][j];
            col[j] += a[i][j];
        }
    }

    let mut ans = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            ans[i][j] = row[i] + col[j] - a[i][j];
        }
    }

    println!("{}", ans.iter().map(|row| row.iter().join(" ")).join("\n"));
}
