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
use rand::Rng;
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
        s: Chars,
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in (0..n - 1).rev() {
        for j in (i + 1..n).rev() {
            if s[i] == s[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1;
            }
        }
    }

    let mut ans = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            ans = ans.max(dp[i][j].min(j - i));
        }
    }

    println!("{}", ans);
}
