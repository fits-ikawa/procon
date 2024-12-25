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
        txa: [(usize, usize, usize); n],
    }

    let tmax = txa.iter().map(|(t, _, _)| t).max().copied().unwrap();

    let txa = txa
        .into_iter()
        .map(|(t, x, a)| (t, (x, a)))
        .collect::<BTreeMap<_, _>>();

    let mut dp = vec![vec![None; 5]; tmax + 1];
    dp[0][0] = Some(0);

    for i in 1..=tmax {
        if let Some(&(x, a)) = txa.get(&i) {
            // すぬけ君が現れるので叩けるなら叩く
            for j in 0..=4_usize {
                for k in j.saturating_sub(1)..=(j + 1).min(4) {
                    dp[i][j] = dp[i][j].max(dp[i - 1][k].map(|e| e + if j == x { a } else { 0 }));
                }
            }
        } else {
            // 移動のみ
            for j in 0..=4_usize {
                for k in j.saturating_sub(1)..=(j + 1).min(4) {
                    dp[i][j] = dp[i][j].max(dp[i - 1][k]);
                }
            }
        }
    }

    println!("{}", dp[tmax].iter().flatten().max().unwrap());
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
