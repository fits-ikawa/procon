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
        s: Chars,
    }

    use mylib::OptionExt;

    // dp[i][j]
    // 負けないようにジャンケンし、i 回目の手が
    // j （0: R, 1: S, 2: P）だった時の最大勝ち数
    let mut dp = vec![vec![None; 3]; n + 1];
    dp[0] = vec![Some(0); 3];

    for i in 1..=n {
        let aoki = s[i - 1];

        dp[i][0] = match aoki {
            'R' => dp[i - 1][1].max_or(dp[i - 1][2]),
            'S' => dp[i - 1][1].max_or(dp[i - 1][2]).map(|x| x + 1),
            'P' => None,
            _ => unreachable!(),
        };

        dp[i][1] = match aoki {
            'R' => None,
            'S' => dp[i - 1][0].max_or(dp[i - 1][2]),
            'P' => dp[i - 1][0].max_or(dp[i - 1][2]).map(|x| x + 1),
            _ => unreachable!(),
        };

        dp[i][2] = match aoki {
            'R' => dp[i - 1][0].max_or(dp[i - 1][1]).map(|x| x + 1),
            'S' => None,
            'P' => dp[i - 1][0].max_or(dp[i - 1][1]),
            _ => unreachable!(),
        };
    }

    println!("{}", dp[n].iter().flatten().max().unwrap());
}

pub mod mylib {
    use std::cmp::Ord;

    pub trait OptionExt<T> {
        /// Returns the minimum of two `Option` values.
        /// If both are `Some`, returns the minimum value. If one is `None`, returns the other.
        fn min_or(self, other: Option<T>) -> Option<T>;

        /// Returns the maximum of two `Option` values.
        /// If both are `Some`, returns the maximum value. If one is `None`, returns the other.
        fn max_or(self, other: Option<T>) -> Option<T>;
    }

    impl<T: Ord> OptionExt<T> for Option<T> {
        fn min_or(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(x), Some(y)) => Some(x.min(y)),
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        }

        fn max_or(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(x), Some(y)) => Some(x.max(y)),
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        }
    }
}
