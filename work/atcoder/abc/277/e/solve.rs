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
        uva: [(Usize1, Usize1, usize); m],
        s: [Usize1; k],
    }

    use mylib::OptionExt;

    let s = s.into_iter().collect::<HashSet<_>>();
    let mut adj = vec![vec![]; n * 2];

    for i in 0..n {
        if s.contains(&i) {
            adj[i].push(i + n);
            adj[i + n].push(i);
        }
    }

    for (u, v, a) in uva {
        if a == 1 {
            adj[u].push(v);
            adj[v].push(u);
        } else {
            adj[u + n].push(v + n);
            adj[v + n].push(u + n);
        }
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![None; n * 2];

    todo.push_back(0);
    seen[0] = Some(0);

    while let Some(from) = todo.pop_front() {
        for &to in &adj[from] {
            if seen[to].is_none() {
                if from.abs_diff(to) == n {
                    // スイッチ切り替え（コスト 0）
                    seen[to] = seen[from];
                    todo.push_front(to);
                } else {
                    // 移動（コスト 1）
                    seen[to] = seen[from].map(|e| e + 1);
                    todo.push_back(to);
                }
            }
        }
    }

    if let Some(ans) = seen[n - 1].min_or(seen[n * 2 - 1]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

pub mod mylib {
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
