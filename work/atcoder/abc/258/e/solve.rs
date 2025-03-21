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
        n: usize, q: usize, x: usize,
        w: [usize; n],
        k: [usize; q],
    }

    let acc = std::iter::once(0)
        .chain(w.clone())
        .chain(w)
        .cumsum::<usize>()
        .collect_vec();

    let sum = acc[n];
    let xx = x % sum;

    if xx == 0 {
        // 割り切れるなら面倒なことはしなくていい
        let ans = (x / sum) * n;

        for _ in 0..q {
            println!("{}", ans);
        }

        return;
    }

    let mut a = vec![0; n];

    for i in 0..n {
        let pos = acc.lower_bound(&(acc[i] + xx));
        a[i] = pos % n;
    }

    const KLOG: usize = 40;

    // ダブリング
    let mut dp = vec![vec![0; n]; KLOG];

    dp[0] = a;

    for i in 1..KLOG {
        for j in 0..n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    for ki in k {
        let mut cur = 0;

        for i in 0..KLOG {
            if (ki - 1) >> i & 1 > 0 {
                cur = dp[i][cur];
            }
        }

        let to = dp[0][cur] + if cur < dp[0][cur] { 0 } else { n };

        println!("{}", (x / sum) * n + to - cur);
    }
}
