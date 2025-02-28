#![allow(clippy::comparison_chain)]
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
        n: usize, k: usize,
        x: [Usize1; n],
        a: [usize; n],
    }

    let logk = k.next_power_of_two().ilog2() as usize;

    // ダブリング
    let mut dp = vec![vec![0; n]; logk + 1];
    dp[0] = x;

    for i in 1..=logk {
        for j in 0..n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut ans = vec![0; n];

    for j in 0..n {
        let mut cur = j;
        for i in 0..=logk {
            if k >> i & 1 > 0 {
                cur = dp[i][cur];
            }
        }
        ans[j] = a[cur];
    }

    println!("{}", ans.iter().join(" "));
}
