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
    // bitDP
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    if n == 1 {
        println!("{}", a[0][0]);
        return;
    }

    use mylib::OptionExt;

    let mut dislike = vec![vec![false; n]; n];

    for (x, y) in xy {
        dislike[x][y] = true;
        dislike[y][x] = true;
    }

    // dp[s][j]
    // 集合 s の選手が区間 |s| までを走り、最後に走ったのが選手 j だったときの最小時間
    let mut dp = vec![vec![None; n]; 1 << n];
    for i in 0..n {
        dp[0][i] = Some(0);
    }

    for s in 1..1_usize << n {
        let i = s.count_ones() as usize;
        for j in 0..n {
            if s & 1 << j > 0 {
                for k in 0..n {
                    if j != k && !dislike[j][k] {
                        dp[s][j] = dp[s][j].min_or(dp[s & !(1 << j)][k].map(|x| x + a[j][i - 1]));
                    }
                }
            }
        }
    }

    let ans = dp[(1 << n) - 1].iter().flatten().min().copied();

    if let Some(ans) = ans {
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

#[allow(dead_code)]
fn solve() {
    // 全探索
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut dislike = vec![vec![false; n]; n];

    for (x, y) in xy {
        dislike[x][y] = true;
        dislike[y][x] = true;
    }

    let mut ans = vec![];

    for perm in (0..n).permutations(n) {
        if perm.windows(2).all(|w| !dislike[w[0]][w[1]]) {
            ans.push(
                perm.iter()
                    .enumerate()
                    .map(|(i, &pi)| a[pi][i])
                    .sum::<usize>(),
            );
        }
    }

    if ans.is_empty() {
        println!("-1");
    } else {
        println!("{}", ans.iter().min().unwrap());
    }
}
