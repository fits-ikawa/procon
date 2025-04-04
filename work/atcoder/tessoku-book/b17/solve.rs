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
        h: [usize; n],
    }

    let mut dp = vec![0; n];
    dp[1] = h[0].abs_diff(h[1]);

    for i in 2..n {
        dp[i] = (dp[i - 1] + h[i - 1].abs_diff(h[i])).min(dp[i - 2] + h[i - 2].abs_diff(h[i]));
    }

    let mut cur = n - 1;
    let mut restore = vec![cur + 1];

    while cur > 0 {
        let c = h[cur].abs_diff(h[cur - 1]);
        if dp[cur] >= c && dp[cur] - c == dp[cur - 1] {
            cur -= 1;
        } else {
            cur -= 2;
        }

        restore.push(cur + 1);
    }

    println!("{}", restore.len());
    println!("{}", restore.iter().rev().join(" "));
}
