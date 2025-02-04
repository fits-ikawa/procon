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
    // dp テーブルの j に HashMap を使用（こっちの方が遅い）
    input! {
        n: usize, k: usize, p: usize,
        ca: [(usize, [usize; k]); n],
    }

    use mylib::OptionExt;

    // dp[i][j]
    // i 番目までの開発案から選んで実行し、パラメータの組み合わせが j になったときのコストの最小値
    let mut dp = vec![hashmap! {}; n + 1];
    dp[0].insert(vec![0_usize; k], Some(0_usize));

    for i in 0..n {
        let (c, a) = &ca[i];
        let vc = dp[i].iter().map(|(v, &c)| (v.clone(), c)).collect_vec();

        for (vs, cost) in vc {
            // i 番目の案を実行しない場合
            let &old_cost = dp[i + 1].get(&vs).unwrap_or(&None);
            dp[i + 1].insert(vs.clone(), old_cost.min_or(cost));

            // i 番目の案を実行する場合
            let ws = izip!(&vs, a).map(|(a, b)| (a + b).min(p)).collect_vec();
            let &old_cost = dp[i + 1].get(&ws).unwrap_or(&None);
            dp[i + 1].insert(ws, old_cost.min_or(cost.map(|e| e + c)));
        }
    }

    if let Some(Some(ans)) = dp[n].get(&vec![p; k]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[allow(dead_code)]
fn solve() {
    // dp テーブルの j に整数を使用
    input! {
        n: usize, k: usize, p: usize,
        ca: [(usize, [usize; k]); n],
    }

    use mylib::OptionExt;

    // dp[i][j]
    // i 番目までの開発案から選んで実行し、パラメータの組み合わせが j になったときのコストの最小値
    let mut dp = vec![vec![None; 1 << (k * 3)]; n + 1];
    dp[0][0] = Some(0);

    let dec = |code| {
        let mut ret = vec![];
        for i in 0..k {
            let m = (code >> (i * 3)) & 0b111;
            if m > p {
                return None;
            }
            ret.push(m);
        }
        Some(ret)
    };

    let enc = |vs: &Vec<usize>| {
        let mut code = 0;
        for i in 0..k {
            assert!(vs[i] <= p);
            code += vs[i] << (i * 3);
        }
        code
    };

    for i in 1..=n {
        let (c, a) = &ca[i - 1];
        for j in 0..1_usize << (k * 3) {
            if let Some(vs) = dec(j) {
                // i 番目の案を実行しない場合
                dp[i][j] = dp[i][j].min_or(dp[i - 1][j]);

                let ws = izip!(&vs, a).map(|(a, b)| (a + b).min(p)).collect_vec();
                let code = enc(&ws);

                dp[i][code] = dp[i][code].min_or(dp[i - 1][j].map(|e| e + c));
            }
        }
    }

    if let Some(ans) = dp[n][enc(&vec![p; k])] {
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
