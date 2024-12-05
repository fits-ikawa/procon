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

    use ac_library::ModInt1000000007 as Mint;

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![None; n]; // コストを記録
    let mut count = vec![Mint::new(0); n];
    todo.push_back(0);
    seen[0] = Some(0);
    count[0] = Mint::new(1);

    while let Some(from) = todo.pop_front() {
        for &to in &adj[from] {
            if seen[to].is_none() {
                seen[to] = seen[from].map(|x| x + 1);
                count[to] = count[from];
                todo.push_back(to);
            } else if seen[from].unwrap() + 1 == seen[to].unwrap() {
                count[to] = count[to] + count[from];
            }
        }
    }

    println!("{}", count[n - 1]);
}
