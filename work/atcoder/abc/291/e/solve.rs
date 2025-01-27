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
        xy: [(Usize1, Usize1); m],
    }

    // トポロジカルソート
    let mut adj = vec![vec![]; n];
    let mut deg = vec![0; n];

    for (x, y) in xy {
        adj[x].push(y);
        deg[y] += 1;
    }

    let mut todo = VecDeque::new();

    for i in 0..n {
        if deg[i] == 0 {
            todo.push_back(i);
        }
    }

    // dp[i]
    // 頂点 i を最後に訪れる最長路のコスト
    let mut dp = vec![0; n];
    let mut order = vec![];

    while let Some(from) = todo.pop_front() {
        order.push(from);

        for &to in &adj[from] {
            dp[to] = dp[from] + 1;
            deg[to] -= 1;
            if deg[to] == 0 {
                todo.push_back(to);
            }
        }
    }

    if dp.iter().max().copied().unwrap() == n - 1 {
        // 最長路が全ての頂点を訪れるなら A を決定できる
        let mut ans = vec![0; n];
        for (i, v) in order.into_iter().enumerate() {
            ans[v] = i + 1;
        }

        println!("Yes");
        println!("{}", ans.iter().join(" "));
    } else {
        println!("No");
    }
}
