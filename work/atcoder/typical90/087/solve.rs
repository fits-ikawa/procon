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
        n: usize, p: usize, k: usize,
        a: [[isize; n]; n],
    }

    const MAX_X: usize = 2000000000;

    let l = {
        let mut left = 0;
        let mut right = MAX_X;

        while right - left > 1 {
            let mid = (left + right) / 2;

            if check(mid, n, p, &a) <= k {
                right = mid;
            } else {
                left = mid;
            }
        }
        right
    };

    let r = {
        let mut left = 0;
        let mut right = MAX_X;

        while right - left > 1 {
            let mid = (left + right) / 2;

            if check(mid, n, p, &a) < k {
                right = mid;
            } else {
                left = mid;
            }
        }
        right
    };

    if l == r {
        println!("0");
    } else if r == MAX_X {
        println!("Infinity");
    } else {
        println!("{}", r - l);
    }
}

fn check(x: usize, n: usize, p: usize, a: &[Vec<isize>]) -> usize {
    let mut dp = vec![vec![usize::MAX; n]; n];

    for i in 0..n {
        for j in 0..n {
            dp[i][j] = if a[i][j] == -1 { x } else { a[i][j] as usize }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(dp[k][j]));
            }
        }
    }

    let mut ret = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            if dp[i][j] <= p {
                ret += 1;
            }
        }
    }

    ret
}
