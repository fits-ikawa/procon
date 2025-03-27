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
        h: usize, w: usize, k: usize,
        x1: usize, y1: usize, x2: usize, y2: usize,
    }

    use ac_library::ModInt998244353 as Mint;

    // dp[i][j]
    // i 手目で状態 j のときの通り数
    // j=0: 目標マス（x2, y2）にいる
    //   1: 目標マスではないが同じ行にいる
    //   2: 目標マスではないが同じ列にいる
    //   3: 目標マスとは別の行、別の列にいる
    let mut dp = vec![vec![Mint::new(0); 4]; k + 1];

    if (x1, y1) == (x2, y2) {
        dp[0][0] = Mint::new(1);
    } else if x1 == x2 {
        dp[0][1] = Mint::new(1);
    } else if y1 == y2 {
        dp[0][2] = Mint::new(1);
    } else {
        dp[0][3] = Mint::new(1);
    }

    for i in 1..=k {
        dp[i][0] = dp[i - 1][1] + dp[i - 1][2];
        dp[i][1] = dp[i - 1][0] * (w - 1) + dp[i - 1][1] * (w - 2) + dp[i - 1][3];
        dp[i][2] = dp[i - 1][0] * (h - 1) + dp[i - 1][2] * (h - 2) + dp[i - 1][3];
        dp[i][3] = dp[i - 1][1] * (h - 1) + dp[i - 1][2] * (w - 1) + dp[i - 1][3] * (h - 2 + w - 2);
    }

    println!("{}", dp[k][0]);
}
