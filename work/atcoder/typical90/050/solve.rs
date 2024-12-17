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
    input! {
        n: usize, l: usize,
    }

    use ac_library::ModInt1000000007 as Mint;

    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] = Mint::new(1);

    for i in 1..=n {
        dp[i] = dp[i] + dp[i - 1];
        if i >= l {
            dp[i] = dp[i] + dp[i - l];
        }
    }

    println!("{}", dp[n]);
}
