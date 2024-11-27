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
        h: usize, w: usize, c:usize,
        mut a: [[usize; w]; h],
    }

    let min_cost = |a: &[Vec<_>]| {
        // dp[i][j]
        // 始点の駅を建てたあと、右下方向に (i, j) まで線路を引いた時の最小コスト
        // (i, j) に始点の駅を建てたケースを含む
        let mut dp = vec![vec![usize::MAX; w]; h];
        dp[0][0] = a[0][0];

        let mut ret = usize::MAX;

        for i in 0..h {
            for j in 0..w {
                if i > 0 {
                    dp[i][j] = dp[i][j].min((dp[i - 1][j] + c).min(a[i][j]));
                    ret = ret.min(dp[i - 1][j] + c + a[i][j]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].min((dp[i][j - 1] + c).min(a[i][j]));
                    ret = ret.min(dp[i][j - 1] + c + a[i][j]);
                }
            }
        }

        ret
    };

    let mut ans = min_cost(&a);
    // 逆方向でも計算する
    a.reverse();
    ans = ans.min(min_cost(&a));

    println!("{}", ans);
}
