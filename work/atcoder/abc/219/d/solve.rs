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
        x: usize, y: usize,
        ab: [(usize, usize); n],
    }

    use mylib::OptionExt;

    // dp[i][j][k]
    // i 番目までの弁当から選んで買い、たこ焼きが j 個、たい焼きが k 個になったときの弁当の最小個数。
    // ただし j == x, k == y については、たこ焼きが x 個「以上」、たい焼きが y 個「以上」になる買い方を表す
    // そのような買い方ができない場合は None
    let mut dp = vec![vec![vec![None; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = Some(0);

    for i in 0..n {
        let (ai, bi) = ab[i];
        for j in 0..=x {
            for k in 0..=y {
                if dp[i][j][k].is_none() {
                    continue;
                }
                dp[i + 1][j][k] = dp[i + 1][j][k].min_or(dp[i][j][k]);

                let j2 = (j + ai).min(x);
                let k2 = (k + bi).min(y);
                dp[i + 1][j2][k2] = dp[i + 1][j2][k2].min_or(dp[i][j][k].map(|e| e + 1));
            }
        }
    }

    if let Some(ans) = dp[n][x][y] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

pub mod mylib {
    use std::cmp::Ord;

    pub trait OptionExt<T> {
        /// Returns the maximum of two `Option` values.
        ///
        /// If both are `Some`, returns the greater value. If one is `None`, returns the other.
        /// If both are `None`, returns `None`.
        ///
        /// # Notes
        /// - This method relies on `PartialOrd`, which means it can handle types like `f64`.
        /// - For types like `f64`, be aware that `NaN` is not comparable. If either value is `NaN`,
        ///   it may result in unexpected behavior.
        ///   Consider pre-processing your data if `NaN` values are possible.
        fn min_or(self, other: Option<T>) -> Option<T>;

        /// Returns the minimum of two `Option` values.
        ///
        /// If both are `Some`, returns the smaller value. If one is `None`, returns the other.
        /// If both are `None`, returns `None`.
        ///
        /// # Notes
        /// - This method relies on `PartialOrd`, which means it can handle types like `f64`.
        /// - For types like `f64`, be aware that `NaN` is not comparable. If either value is `NaN`,
        ///   it may result in unexpected behavior.
        ///   Consider pre-processing your data if `NaN` values are possible.
        fn max_or(self, other: Option<T>) -> Option<T>;
    }

    impl<T: PartialOrd> OptionExt<T> for Option<T> {
        fn max_or(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(x), Some(y)) => {
                    if x > y {
                        Some(x)
                    } else {
                        Some(y)
                    }
                }
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        }

        fn min_or(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(x), Some(y)) => {
                    if x < y {
                        Some(x)
                    } else {
                        Some(y)
                    }
                }
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        }
    }
}
