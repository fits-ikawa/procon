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
        n: usize,
        s: Bytes,
    }

    let s = s.into_iter().map(|si| (si - b'A') as usize).collect_vec();

    use ac_library::ModInt998244353 as Mint;

    // dp[i][j][k]
    // i 回目までのコンテストから集合 j の種類に出場し、最後に出場したコンテストが k のときの通り数
    let mut dp = vec![[[Mint::new(0); 10]; 1 << 10]; n + 1];

    for i in 1..=n {
        let si = s[i - 1];

        for j in 0..1 << 10 {
            for k in 0..10 {
                // 今回 si に出場しない
                dp[i][j][k] = dp[i][j][k] + dp[i - 1][j][k];

                // すでに出場したコンテストがあり、si に初めて出場する
                if j >> si & 1 == 0 {
                    dp[i][j + (1 << si)][si] = dp[i][j + (1 << si)][si] + dp[i - 1][j][k];
                }
            }

            // 1 回目の出場として si を選ぶ
            if j == 0 {
                dp[i][1 << si][si] += 1;
            }

            // 過去に si に出場したことがあって、今回の si にも出場する
            if j >> si & 1 > 0 {
                dp[i][j][si] = dp[i][j][si] + dp[i - 1][j][si];
            }
        }
    }

    let ans = iproduct!((0..1 << 10), (0..10))
        .map(|(j, k)| dp[n][j][k])
        .sum::<Mint>();

    println!("{}", ans);
}
