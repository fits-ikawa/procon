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
        n: usize, p: usize,
    }

    use ac_library::ModInt998244353 as Mint;

    let mut dp = vec![Mint::new(0); n + 1];
    dp[1] = Mint::new(1);

    for i in 2..=n {
        let hit2 = (dp[i - 2] + 1) * (Mint::new(p) / 100);
        let hit1 = (dp[i - 1] + 1) * (Mint::new(100 - p) / 100);

        dp[i] = hit2 + hit1;
    }

    println!("{}", dp[n]);
}
