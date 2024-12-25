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
        let (ai, bi) = ab[i - 1];
        for j in 0..=s {
            if j >= ai {
                dp[i][j] = dp[i][j] || dp[i - 1][j - ai];
            }
            if j >= bi {
                dp[i][j] = dp[i][j] || dp[i - 1][j - bi];
            }
        }
    }

    if dp[n][s] {
        let mut ans = vec![];
        let mut cur = s;

        for i in (0..n).rev() {
            let (ai, bi) = ab[i];
            if cur >= ai && dp[i][cur - ai] {
                ans.push("H");
                cur -= ai;
            } else if cur >= bi && dp[i][cur - bi] {
                ans.push("T");
                cur -= bi;
            }
        }

        println!("Yes");
        println!("{}", ans.iter().rev().join(""));
    } else {
        println!("No");
    }
}
