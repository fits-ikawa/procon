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
        uv: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    // 探索開始点として葉を一つ選ぶ
    let mut start = 0;

    for i in 0..n {
        if adj[i].len() == 1 {
            start = i;
            break;
        }
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![None; n]; // コスト mod 3 を記録

    todo.push_back(start);
    seen[start] = Some(0);

    while let Some(from) = todo.pop_front() {
        for &to in &adj[from] {
            if seen[to].is_none() {
                seen[to] = seen[from].map(|e| (e + 1) % 3);
                todo.push_back(to);
            }
        }
    }

    let mut ans = vec![];

    for i in 0..n {
        if seen[i].unwrap() == 1 {
            // 1 が記録された頂点が星の中心
            ans.push(adj[i].len());
        }
    }

    ans.sort();

    println!("{}", ans.iter().join(" "));
}
