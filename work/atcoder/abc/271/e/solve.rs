#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        n: usize, m: usize, k: usize,
        abc: [(Usize1, Usize1, usize); m],
        e: [Usize1; k],
    }

    use mylib::OptionExt;

    let mut dp = vec![None; n];
    dp[0] = Some(0);

    for ei in e {
        let (a, b, c) = abc[ei];

        dp[b] = dp[b].min_or(dp[a].map(|e| e + c));
    }

    if let Some(ans) = dp[n - 1] {
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
