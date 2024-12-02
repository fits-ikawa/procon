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
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut adj = vec![vec![false; n]; n];
    let mut cost = vec![vec![0; n]; n];

    for &(a, b, c) in &abc {
        adj[a][b] = true;
        adj[b][a] = true;
        cost[a][b] = c;
        cost[b][a] = c;
    }

    let mut path = (0..n).collect_vec();
    let mut ans = 0;

    loop {
        let mut dist = 0;
        for i in 0..n - 1 {
            let from = path[i];
            let to = path[i + 1];
            if !adj[from][to] {
                // 道がつながってない
                break;
            }
            dist += cost[from][to];
        }
        ans = ans.max(dist);

        if !path.next_permutation() {
            break;
        }
    }

    println!("{}", ans);
}
