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
        ab: [(usize, usize); n],
    }

    use ac_library::ModInt998244353 as Mint;

    // dp[i][j]
    // i 番目までのカードを並べ、i 番目のカードの面が
    // j （0: 表, 1: 裏）だったときの条件を満たす通り数
    let mut dp = vec![vec![Mint::new(0); 2]; n + 1];
    dp[1][0] = Mint::new(1);
    dp[1][1] = Mint::new(1);

    for i in 2..=n {
        let (a, b) = ab[i - 1];
        let (prev_a, prev_b) = ab[i - 2];

        if prev_a != a {
            dp[i][0] = dp[i][0] + dp[i - 1][0];
        }
        if prev_b != a {
            dp[i][0] = dp[i][0] + dp[i - 1][1];
        }
        if prev_a != b {
            dp[i][1] = dp[i][1] + dp[i - 1][0];
        }
        if prev_b != b {
            dp[i][1] = dp[i][1] + dp[i - 1][1];
        }
    }

    println!("{}", dp[n][0] + dp[n][1]);
}
