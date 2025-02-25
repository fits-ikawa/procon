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
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    // 処理の簡単のため、次数 1 の頂点を根として選ぶ
    let root = (0..n).find(|&i| adj[i].len() == 1).unwrap_or(0);

    // dp[v]
    // root を根とする木の v (!= root) を根とする部分木で
    // v の親を追加して広義アルカン（次数 4 の頂点がなくてもいい）になるよう
    // 頂点を選んだときの（親を含まない）最大頂点数
    let mut dp = vec![1; n];
    let mut ans = 0;

    dfs(root, usize::MAX, &mut dp, &mut ans, &adj);

    if ans >= 5 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn dfs(v: usize, p: usize, dp: &mut [usize], ans: &mut usize, adj: &[Vec<usize>]) {
    let mut xs = vec![];

    for &w in &adj[v] {
        if w != p {
            dfs(w, v, dp, ans, adj);
            xs.push(dp[w]);
        }
    }

    xs.sort();
    xs.reverse();

    if adj[v].len() >= 4 {
        dp[v] += xs.iter().take(3).sum::<usize>();
    }

    if adj[v].len() >= 5 {
        // 次数が 5 以上なら親を含まないアルカンを構成できる
        // （親+三つの子の場合の頂点数以上に必ずできる）
        *ans = (*ans).max(dp[v] + xs[3]);
    } else if p != usize::MAX {
        // 次数が 4 以下なら親を含めて（広義）アルカンを構成する
        *ans = (*ans).max(dp[v] + 1);
    }
}
