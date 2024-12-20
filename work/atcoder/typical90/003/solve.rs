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
        ab: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    let cost = bfs(0, &adj, n);
    let farthest = cost
        .into_iter()
        .enumerate()
        .max_by_key(|&(_, x)| x)
        .unwrap()
        .0;

    let cost = bfs(farthest, &adj, n);

    println!("{}", cost.iter().max().unwrap() + 1);
}

fn bfs(start: usize, adj: &[Vec<usize>], n: usize) -> Vec<usize> {
    let mut todo = VecDeque::new();
    let mut seen = vec![None; n];
    todo.push_back(start);
    seen[start] = Some(0);

    while let Some(from) = todo.pop_front() {
        for &to in &adj[from] {
            if seen[to].is_none() {
                seen[to] = seen[from].map(|x| x + 1);
                todo.push_back(to);
            }
        }
    }

    seen.iter().flatten().copied().collect()
}
