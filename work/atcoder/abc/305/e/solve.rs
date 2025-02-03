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
        ab: [(Usize1, Usize1); m],
        ph: [(Usize1, usize); k],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut todo = BinaryHeap::new();
    let mut seen = vec![None; n];

    for (p, h) in ph {
        todo.push((h, p));
        seen[p] = Some(h);
    }

    while let Some((h, from)) = todo.pop() {
        if h > 0 {
            for &to in &adj[from] {
                if seen[to].is_none() || seen[to].unwrap() < h - 1 {
                    seen[to] = Some(h - 1);
                    todo.push((h - 1, to));
                }
            }
        }
    }

    let ans = (0..n)
        .filter_map(|i| if seen[i].is_some() { Some(i + 1) } else { None })
        .collect_vec();

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
