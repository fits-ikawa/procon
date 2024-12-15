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
        ab: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![None; n]; // コストを記録
    todo.push_back(0);
    seen[0] = Some(0);

    while let Some(from) = todo.pop_front() {
        for &to in &adj[from] {
            if seen[to].is_none() {
                seen[to] = seen[from].map(|x| x + 1);
                todo.push_back(to);
            } else if to == 0 {
                println!("{}", seen[from].unwrap() + 1);
                return;
            }
        }
    }

    println!("-1");
}
