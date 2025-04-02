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
    }

    use ac_library::ModInt998244353 as Mint;

    let mint0 = Mint::new(0);
    let mint1 = Mint::new(1);
    let divm = mint1 / m;

    let mut dp = vec![mint0; n + 1];
    dp[0] = mint1;

    for _ in 0..k {
        let mut next_dp = vec![mint0; n + 1];

        for i in 0..n {
            if dp[i] != mint0 {
                for j in 1..=m {
                    let to = if i + j > n { n - (i + j - n) } else { i + j };
                    next_dp[to] += dp[i] * divm;
                }
            }
        }

        dp = next_dp;
    }

    let ans = mint1 - dp[..n].iter().sum::<Mint>();

    println!("{}", ans);
}
