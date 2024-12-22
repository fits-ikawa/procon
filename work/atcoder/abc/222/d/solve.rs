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
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    use ac_library::ModInt998244353 as Mint;

    let m = b.iter().max().unwrap();

    // dp[i][j]
    // i 番目までの数列 c で i 番目の数が j のときの通り数
    let mut dp = vec![vec![Mint::new(0); m + 1]; n + 1];
    for j in a[0]..=b[0] {
        dp[1][j] = Mint::new(1);
    }

    for i in 2..=n {
        let ai = a[i - 1];
        let bi = b[i - 1];

        let mut acc = vec![Mint::new(0); m + 2];
        for j in 1..=m + 1 {
            acc[j] = acc[j - 1] + dp[i - 1][j - 1];
        }

        for j in ai..=bi {
            dp[i][j] = acc[j + 1] - acc[a[i - 2]];
        }
    }

    println!("{}", dp[n].iter().sum::<Mint>());
}
