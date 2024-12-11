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
        x: usize, y: usize, z: usize,
        s: Chars,
    }

    // dp[i][j]
    // i 文字目までを打ち終えて CapsLock の状態が j （0: OFF, 1: ON）
    // のときの最小経過時間
    let mut dp = vec![vec![0; 2]; s.len() + 1];
    dp[0][1] = z;

    for i in 1..=s.len() {
        let si = s[i - 1];

        dp[i][0] = (dp[i - 1][0] + if si == 'a' { x } else { y })
            .min(dp[i - 1][1] + z + if si == 'a' { x } else { y });

        dp[i][1] = (dp[i - 1][0] + z + if si == 'A' { x } else { y })
            .min(dp[i - 1][1] + if si == 'A' { x } else { y });
    }

    println!("{}", dp[s.len()].iter().min().unwrap());
}
