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
        n: usize, m: usize,
        a: [[usize; n]; m],
    }

    use mylib::OptionExt;

    let a = a
        .into_iter()
        .map(|ai| {
            ai.into_iter()
                .enumerate()
                .map(|(j, aij)| aij << j)
                .sum::<usize>()
        })
        .collect_vec();

    // dp[i][s]
    // i 枚目までのクーポンから何枚か使用し、集合 s の商品を無料にしたときの最小使用クーポン数
    let mut dp = vec![vec![None; 1 << n]; m + 1];
    dp[0][0] = Some(0);

    for i in 1..=m {
        let ai = a[i - 1];
        for s in 0..1 << n {
            dp[i][s] = dp[i][s].min_or(dp[i - 1][s]);
            dp[i][s | ai] = dp[i][s | ai].min_or(dp[i - 1][s].map(|x| x + 1));
        }
    }

    if let Some(ans) = dp[m][(1 << n) - 1] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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
