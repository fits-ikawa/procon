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
    }

    let mut c = vec![];
    let mut p = vec![];

    for _ in 0..n {
        input! {
            ci: usize,
            pi: [Usize1; ci],
        }

        c.push(ci);
        p.push(pi);
    }

    let mut adj = vec![vec![]; n];
    let mut adj_rev = vec![vec![]; n];
    let mut indeg = vec![0; n];

    for (i, pi) in p.into_iter().enumerate() {
        indeg[i] = pi.len();

        for pij in pi {
            adj[pij].push(i);
            adj_rev[i].push(pij);
        }
    }

    // 本 1 を読む前に読むべき本の集合 need を求める
    let mut todo = VecDeque::new();
    let mut seen = vec![false; n];

    todo.push_back(0);
    seen[0] = true;

    while let Some(from) = todo.pop_front() {
        for &to in &adj_rev[from] {
            if !seen[to] {
                seen[to] = true;
                todo.push_back(to);
            }
        }
    }

    let need = (0..n)
        .filter(|&i| seen[i] && i != 0)
        .collect::<HashSet<_>>();

    // 読むべき本を読む順番 order を求める（トポロジカルソート）
    let mut todo = VecDeque::new();
    let mut order = vec![];

    for i in 0..n {
        if indeg[i] == 0 {
            todo.push_back(i);
        }
    }

    while let Some(from) = todo.pop_front() {
        order.push(from);

        for &to in &adj[from] {
            indeg[to] -= 1;
            if indeg[to] == 0 {
                todo.push_back(to);
            }
        }
    }

    // order のうち need に含まれる本を順番に取り出したものが答え
    let ans = order
        .into_iter()
        .filter_map(|v| if need.contains(&v) { Some(v + 1) } else { None })
        .collect_vec();

    if !ans.is_empty() {
        println!("{}", ans.iter().join(" "));
    }
}
