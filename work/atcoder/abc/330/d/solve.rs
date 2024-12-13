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
        mut s: [Chars; n],
    }

    let mut ans = count(&s, n);

    s.reverse();

    ans += count(&s, n);

    println!("{}", ans);
}

#[allow(clippy::needless_range_loop)]
fn count(s: &[Vec<char>], n: usize) -> usize {
    let row_count = (0..n)
        .map(|i| s[i].iter().filter(|&&si| si == 'o').count())
        .collect_vec();

    let mut acc = vec![vec![0; n]; n];

    for j in 0..n {
        for i in 0..n {
            if i == 0 {
                acc[j][i] = if s[i][j] == 'o' { 1 } else { 0 };
            } else {
                acc[j][i] = acc[j][i - 1] + if s[i][j] == 'o' { 1 } else { 0 };
            }
        }
    }

    let mut ans = 0;

    for i in 0..n - 1 {
        for j in 0..n {
            if s[i][j] == 'o' {
                ans += (acc[j][n - 1] - acc[j][i]) * (row_count[i] - 1);
            }
        }
    }

    ans
}
