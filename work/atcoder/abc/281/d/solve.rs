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
        n: usize, k: usize, d: usize,
        a: [usize; n],
    }

    // dp[i][j][l]
    // a の i 番目までの数字から j 個選んだ総和を d で割った余りが l のときの、総和の最大値
    let mut dp = vec![vec![vec![None; d]; k + 1]; n + 1];
    dp[0][0][0] = Some(0);

    for i in 1..=n {
        let ai = a[i - 1];
        let r = ai % d;

        for j in 0..=k {
            for l in 0..d {
                dp[i][j][l] = dp[i - 1][j][l];
                if j > 0 {
                    let l2 = (l as isize - r as isize).mod_floor(&(d as isize)) as usize;
                    dp[i][j][l] = dp[i][j][l].max(dp[i - 1][j - 1][l2].map(|e| e + ai));
                }
            }
        }
    }

    if let Some(x) = dp[n][k][0] {
        println!("{}", x);
    } else {
        println!("-1");
    }
}
