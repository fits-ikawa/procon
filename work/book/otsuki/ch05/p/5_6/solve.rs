#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize, w: usize,
        nums: [(usize, usize); n],
    }

    // dp[i][j]
    // i 番目までの数で j を作れるとき、i 番目の数を何個残せるか
    // j を作れない場合は None
    let mut dp = vec![vec![None; w + 1]; n + 1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        let (a, m) = nums[i - 1];
        for j in 0..=w {
            if dp[i - 1][j].is_some() {
                // i - 1 番目までで j を作れるなら a を m 個残せる
                dp[i][j] = Some(m);
            } else if j < a || dp[i][j - a].is_none() {
                // i - 1 番目までで j を作れてないのだから
                // j < a では j は作れない
                // j - a が作れてないならやはり j は作れない
                dp[i][j] = None;
            } else {
                // j - a が作れているなら a を 1 個消費することで j を作れる
                // ただし dp[i][j - a] が 0 の場合（a が余ってない場合）は作れない
                // （checked_sub が失敗して None になることで表現）
                dp[i][j] = dp[i][j - a].and_then(|x| x.checked_sub(1));
            }
        }
    }

    println!("{}", if dp[n][w].is_some() { "YES" } else { "NO" });
}
