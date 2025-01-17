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
        n: usize, m: usize, k: usize,
    }

    use ac_library::ModInt998244353 as Mint;

    // dp[i][j]
    // 問題の条件を満たす長さ i の数列であって、最後が j になるものの通り数
    let mut dp = vec![vec![Mint::new(0); m + 1]; n + 1];

    for j in 1..=m {
        dp[1][j] = Mint::new(1);
    }

    for i in 2..=n {
        let mut s = dp[i - 1][1 + k..].iter().sum::<Mint>();

        for j in 1..=m {
            dp[i][j] = s;

            if j + 1 >= k && j + 1 - k <= m {
                s += dp[i - 1][j + 1 - k];
            }
            if j + k <= m {
                s -= dp[i - 1][j + k];
            }
        }
    }

    println!("{}", dp[n].iter().sum::<Mint>());
}
