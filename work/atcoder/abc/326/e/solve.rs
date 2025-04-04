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
        n: usize,
        a: [usize; n],
    }

    use ac_library::ModInt998244353 as Mint;

    let mint0 = Mint::new(0);
    let divn = Mint::new(1) / n;

    let mut dp = vec![mint0; n + 1];
    dp[n] = Mint::new(a[n - 1]);

    let mut acc = dp[n];

    for i in (0..n).rev() {
        let ai = if i == 0 { mint0 } else { Mint::new(a[i - 1]) };

        dp[i] = ai + acc * divn;
        acc += dp[i];
    }

    println!("{}", dp[0]);
}
