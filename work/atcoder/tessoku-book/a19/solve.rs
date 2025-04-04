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
        n: usize, w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![None; w + 1]; n + 1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        let (wi, vi) = wv[i - 1];

        for j in 0..=w {
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);

            if j + wi <= w {
                dp[i][j + wi] = dp[i - 1][j].map(|e| e + vi);
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap().unwrap());
}
