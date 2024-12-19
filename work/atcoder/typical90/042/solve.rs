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
        k: usize,
    }

    use ac_library::ModInt1000000007 as Mint;

    if k % 9 != 0 {
        println!("0");
        return;
    }

    let mut dp = vec![Mint::new(0); k + 1];
    dp[0] = Mint::new(1);

    for i in 0..k {
        for j in 1..=9 {
            if i + j <= k {
                dp[i + j] = dp[i + j] + dp[i];
            }
        }
    }

    println!("{}", dp[k]);
}
