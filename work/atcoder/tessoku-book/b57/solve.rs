#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        n: usize, k: usize,
    }

    const LOGK: usize = 30;

    // ダブリング
    let mut dp = vec![vec![0; n + 1]; LOGK];
    dp[0] = (0..=n).map(f).collect_vec();

    for i in 1..LOGK {
        for j in 0..=n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut ans = Vec::with_capacity(n);

    for m in 1..=n {
        let mut cur = m;

        for i in 0..LOGK {
            if k >> i & 1 > 0 {
                cur = dp[i][cur];
            }
        }

        ans.push(cur);
    }

    println!("{}", ans.iter().join("\n"));
}

fn f(x: usize) -> usize {
    let mut y = x;
    let mut sum = 0;

    while y > 0 {
        sum += y % 10;
        y /= 10;
    }

    x - sum
}
