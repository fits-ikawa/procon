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
        a: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut adj = vec![hashset! {}; n];

    for (u, v) in uv {
        adj[u].insert(v);
        adj[v].insert(u);
    }

    let mut cost = vec![0; n];
    let mut todo = BinaryHeap::new();

    for i in 0..n {
        cost[i] = adj[i].iter().map(|&j| a[j]).sum::<usize>();
        todo.push((Reverse(cost[i]), i));
    }

    let mut removed = hashset! {};
    let mut ans = 0;

    while let Some((Reverse(c), v)) = todo.pop() {
        if removed.contains(&v) {
            continue;
        }

        ans = ans.max(c);
        removed.insert(v);

        for w in adj[v].clone() {
            cost[w] -= a[v];
            adj[w].remove(&v);
            todo.push((Reverse(cost[w]), w));
        }
    }

    println!("{}", ans);
}
