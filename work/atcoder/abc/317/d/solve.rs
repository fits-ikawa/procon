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
        xyz: [(usize, usize, usize); n],
    }

    use mylib::OptionExt;

    let m = xyz.iter().map(|&(_, _, z)| z).sum::<usize>();

    // dp[i][j]
    // i 番目の選挙区までで獲得議席数を j にするために必要な最小の鞍替え数
    let mut dp = vec![vec![None; m + 1]; n + 1];

    for i in 1..=n {
        let (x, y, z) = xyz[i - 1];

        for j in 0..=m {
            dp[i][j] = dp[i - 1][j];

            if j == z {
                if x > y {
                    dp[i][j] = dp[i][j].min_or(Some(0));
                } else {
                    dp[i][j] = dp[i][j].min_or(Some((y - x + 1) / 2));
                }
            } else if j > z {
                if x > y {
                    dp[i][j] = dp[i][j].min_or(dp[i - 1][j - z]);
                } else {
                    dp[i][j] = dp[i][j].min_or(dp[i - 1][j - z].map(|e| e + (y - x + 1) / 2));
                }
            }
        }
    }

    let mut ans = usize::MAX;

    for j in m / 2 + 1..=m {
        if dp[n][j].is_some() {
            ans = ans.min(dp[n][j].unwrap());
        }
    }

    println!("{}", ans);
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

        /// Returns the maximum of two `Option` values if both are `Some`.
        ///
        /// If one is `None`, returns `None`. If both are `Some`, returns the greater value.
        ///
        /// # Notes
        /// - This method relies on `PartialOrd`, which means it can handle types like `f64`.
        /// - For types like `f64`, be aware that `NaN` is not comparable. If either value is `NaN`,
        ///   it may result in unexpected behavior.
        ///   Consider pre-processing your data if `NaN` values are possible.
        fn max_and(self, other: Option<T>) -> Option<T>;
    }

    impl<T: PartialOrd> OptionExt<T> for Option<T> {
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

        fn max_and(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(x), Some(y)) => {
                    if x > y {
                        Some(x)
                    } else {
                        Some(y)
                    }
                }
                _ => None,
            }
        }
    }
}
