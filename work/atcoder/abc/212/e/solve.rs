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
        n: usize, m: usize, k: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut not_adj = vec![vec![]; n];

    for i in 0..n {
        not_adj[i].push(i);
    }

    for (u, v) in uv {
        not_adj[u].push(v);
        not_adj[v].push(u);
    }

    use ac_library::ModInt998244353 as Mint;

    let mut dp = vec![vec![Mint::new(0); n]; k + 1];
    dp[0][0] = Mint::new(1);

    for i in 1..=k {
        let sum = dp[i - 1].iter().sum::<Mint>();
        for j in 0..n {
            dp[i][j] = sum - not_adj[j].iter().map(|&k| dp[i - 1][k]).sum::<Mint>();
        }
    }

    println!("{}", dp[k][0]);
}
