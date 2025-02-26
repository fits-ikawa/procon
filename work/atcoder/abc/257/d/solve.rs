#![allow(clippy::comparison_chain)]
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
        xyp: [(isize, isize, usize); n],
    }

    let mut left = 0;
    let mut right = 5000000000;

    while right - left > 1 {
        let mid = (left + right) / 2;

        // ワーシャルフロイド法
        let mut dp = vec![vec![usize::MAX; n]; n];

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    dp[i][i] = 0;
                } else {
                    let (ix, iy, ip) = xyp[i];
                    let (jx, jy, _) = xyp[j];
                    if mid * ip >= ix.abs_diff(jx) + iy.abs_diff(jy) {
                        dp[i][j] = 1;
                    }
                }
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(dp[k][j]));
                }
            }
        }

        let check = || {
            for i in 0..n {
                if (0..n).all(|j| dp[i][j] < usize::MAX) {
                    return true;
                }
            }
            false
        };

        if check() {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
