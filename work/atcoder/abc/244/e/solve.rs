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
    // 解説 AC
    input! {
        n: usize, m: usize, k: usize, s: Usize1, t: Usize1, x: Usize1,
        uv: [(Usize1, Usize1); m],
    }

    use ac_library::ModInt998244353 as Mint;

    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    // dp[i][j][k]
    // 頂点 s から辺を i 回通って頂点 j へ行き、
    // 道中 x を通った回数 mod 2 が k であるような通り数
    let mut dp = vec![vec![vec![Mint::new(0); 2]; n]; k + 1];
    dp[0][s][0] = Mint::new(1);

    for i in 1..=k {
        for j in 0..n {
            for &from in &adj[j] {
                dp[i][j][0] = dp[i][j][0] + dp[i - 1][from][if j == x { 1 } else { 0 }];
                dp[i][j][1] = dp[i][j][1] + dp[i - 1][from][if j == x { 0 } else { 1 }];
            }
        }
    }

    println!("{}", dp[k][t][0]);
}
