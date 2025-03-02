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
        n: usize, m: usize,
        a: [usize; m],
    }

    use ac_library::ModInt1000000007 as Mint;

    let set = HashSet::<_>::from_iter(a.iter().copied());

    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] = Mint::new(1);

    for i in 1..=n {
        if !set.contains(&i) {
            dp[i] = dp[i - 1];
            if i >= 2 {
                dp[i] = dp[i] + dp[i - 2];
            }
        }
    }

    println!("{}", dp[n]);
}
