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
        n: usize, m: usize,
        x: [usize; n],
        cy: [(usize, usize); m],
    }

    let mut c2y = vec![0; n + 1];
    for (c, y) in cy {
        c2y[c] = y;
    }

    // dp[i][j]
    // i 回目のコイントスでカウンタが j になったときの最大金額
    // j == 0 が裏が出たことを示すので、表裏の次元はいらなかった
    let mut dp = vec![vec![None; n + 1]; n + 1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        let xi = x[i - 1];
        for j in 0..i {
            dp[i][0] = dp[i][0].max(dp[i - 1][j]);
            dp[i][j + 1] = dp[i - 1][j].map(|e| e + xi + c2y[j + 1]);
        }
    }

    println!("{}", dp[n].iter().flatten().max().unwrap());
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, m: usize,
        x: [usize; n],
        cy: [(usize, usize); m],
    }

    let cy = cy.into_iter().collect::<BTreeMap<usize, usize>>();

    // dp[i][j][k]
    // i 回目のコイントスが j（0: 裏, 1: 表）でカウンタが k になった時の最大金額
    let mut dp = vec![vec![vec![None; n + 1]; 2]; n + 1];
    dp[0][0][0] = Some(0);

    for i in 1..=n {
        let xi = x[i - 1];

        for j in 0..n {
            let bonus = cy.get(&(j + 1)).unwrap_or(&0);
            if j == 0 {
                dp[i][0][0] = dp[i - 1][0][0];
                dp[i][1][1] = dp[i - 1][0][0].map(|e| e + xi + bonus);
            } else {
                dp[i][0][0] = dp[i][0][0].max(dp[i - 1][1][j]);
                dp[i][1][j + 1] = dp[i - 1][1][j].map(|e| e + xi + bonus);
            }
        }
    }

    println!("{}", dp[n][1].iter().flatten().max().unwrap());
}
