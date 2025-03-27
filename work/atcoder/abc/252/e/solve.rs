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
        n: usize, m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut adj = vec![vec![]; n];
    let mut r2i = hashmap! {};

    for (i, (a, b, c)) in abc.into_iter().enumerate() {
        adj[a].push((b, c));
        adj[b].push((a, c));

        r2i.insert((a, b), i);
        r2i.insert((b, a), i);
    }

    let mut todo = BinaryHeap::new();
    let mut cost = vec![usize::MAX; n];
    let mut prev = vec![0; n];

    todo.push((Reverse(0), 0));
    cost[0] = 0;

    while let Some((Reverse(c), from)) = todo.pop() {
        if cost[from] < c {
            continue;
        }

        for &(to, d) in &adj[from] {
            if cost[to] > c + d {
                cost[to] = c + d;
                todo.push((Reverse(c + d), to));
                prev[to] = from;
            }
        }
    }

    let ans = (1..n).map(|i| r2i[&(prev[i], i)] + 1).collect_vec();

    println!("{}", ans.iter().join(" "));
}
