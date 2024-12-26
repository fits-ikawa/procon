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
    input! {
        n: usize, s: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        let (a, b) = ab[i - 1];
        for j in 0..=s {
            if j >= a {
                dp[i][j] = dp[i][j] || dp[i - 1][j - a];
            }
            if j >= b {
                dp[i][j] = dp[i][j] || dp[i - 1][j - b];
            }
        }
    }

    if dp[n][s] {
        let mut ans = vec![];
        let mut cur = s;

        for i in (0..n).rev() {
            let (a, b) = ab[i];
            if a <= cur && dp[i][cur - a] {
                ans.push('A');
                cur -= a;
            } else if b <= cur && dp[i][cur - b] {
                ans.push('B');
                cur -= b;
            }
        }

        println!("{}", ans.iter().rev().collect::<String>());
    } else {
        println!("Impossible");
    }
}
