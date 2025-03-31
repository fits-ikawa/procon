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
        n: usize, q: usize,
        a: [Usize1; n],
        xy: [(Usize1, usize); q],
    }

    const LOGY: usize = 30;

    // ダブリング
    let mut dp = vec![vec![0; n]; LOGY];
    dp[0] = a;

    for i in 1..LOGY {
        for j in 0..n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    for (x, y) in xy {
        let mut cur = x;

        for i in 0..LOGY {
            if y >> i & 1 > 0 {
                cur = dp[i][cur];
            }
        }

        println!("{}", cur + 1);
    }
}
