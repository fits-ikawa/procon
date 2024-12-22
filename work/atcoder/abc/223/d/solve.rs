#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use petgraph::algo::toposort;
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
    // トポロジカルソート
    input! {
        n: usize, m: usize,
        mut ab: [(Usize1, Usize1); m],
    }

    let mut indeg = vec![0; n];
    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        indeg[b] += 1;
        adj[a].push(b);
    }

    let mut heap = BinaryHeap::new();

    for v in 0..n {
        if indeg[v] == 0 {
            heap.push(Reverse(v));
        }
    }

    let mut order = vec![];

    while let Some(Reverse(v)) = heap.pop() {
        order.push(v);

        for &next in &adj[v] {
            indeg[next] -= 1;
            if indeg[next] == 0 {
                heap.push(Reverse(next));
            }
        }
    }

    if order.len() == n {
        println!("{}", order.iter().map(|v| v + 1).join(" "));
    } else {
        println!("-1");
    }
}
