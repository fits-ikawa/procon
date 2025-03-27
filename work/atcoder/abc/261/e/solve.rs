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
        n: usize, c: usize,
        ta: [(usize, usize); n],
    }

    let mut dp = vec![[[0; 30]; 2]; n + 1];
    dp[0][1] = [1; 30];

    for i in 1..=n {
        let (t, a) = ta[i - 1];
        for j in 0..=1 {
            for k in 0..30 {
                if t == 1 {
                    dp[i][j][k] = dp[i - 1][j][k] & (a >> k & 1);
                } else if t == 2 {
                    dp[i][j][k] = dp[i - 1][j][k] | (a >> k & 1);
                } else {
                    dp[i][j][k] = dp[i - 1][j][k] ^ (a >> k & 1);
                }
            }
        }
    }

    let mut cur = c;

    for i in 1..=n {
        let mut ans = 0;
        for k in 0..30 {
            ans += dp[i][cur >> k & 1][k] << k;
        }
        println!("{}", ans);
        cur = ans;
    }
}
