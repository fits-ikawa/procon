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
        n: usize, m: usize,
        uvw: [(Usize1, Usize1, isize); m],
    }

    let mut adj = vec![vec![]; n];

    for (u, v, w) in uvw {
        adj[u].push((v, w));
        adj[v].push((u, -w));
    }

    let mut seen = vec![None; n]; // コストを記録

    for start in 0..n {
        if seen[start].is_none() {
            let mut todo = VecDeque::new();
            todo.push_back(start);
            seen[start] = Some(0);

            while let Some(from) = todo.pop_front() {
                for &(to, w) in &adj[from] {
                    if seen[to].is_none() {
                        seen[to] = seen[from].map(|x| x + w);
                        todo.push_back(to);
                    }
                }
            }
        }
    }

    println!("{}", seen.iter().map(|x| x.unwrap_or(0)).join(" "));
}
