#![allow(unused_imports)]
use core::f64;
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
        xy: [(usize, usize); n],
    }

    use mylib::OptionExt;

    // dp[s][i]
    // 集合 s の街を一度ずつ巡って町 i にいるときの最小移動距離
    let mut dp = vec![vec![None; n]; 1 << n];
    dp[0][0] = Some(0.0); // スタート地点に戻ってくるので、町 0 にいるが町 0 は s に含まない

    for s in 0..1 << n {
        for i in 0..n {
            if dp[s][i].is_none() {
                continue;
            }

            for j in 0..n {
                let next = 1 << j;
                if i != j && s & next == 0 {
                    dp[s | next][j] =
                        dp[s | next][j].min_or(dp[s][i].map(|x| x + dist(xy[i], xy[j])));
                }
            }
        }
    }

    println!("{}", dp[(1 << n) - 1][0].unwrap());
}

fn dist(a: (usize, usize), b: (usize, usize)) -> f64 {
    ((a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2)) as f64).sqrt()
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
