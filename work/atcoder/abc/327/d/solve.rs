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
    // UnionFind で二部グラフ判定
    input! {
        n: usize, m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    }

    let mut uf = ac_library::Dsu::new(n * 2);

    for (ai, bi) in izip!(a, b) {
        uf.merge(ai, bi + n);
        uf.merge(ai + n, bi);
    }

    for i in 0..n {
        if uf.same(i, i + n) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

#[allow(dead_code)]
fn solve() {
    // BFS で二部グラフ判定
    input! {
        n: usize, m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    }

    let mut adj = vec![hashset! {}; n];

    for (ai, bi) in izip!(a, b) {
        adj[ai].insert(bi);
        adj[bi].insert(ai);
    }

    let mut seen = vec![None; n]; // 偶奇を記録

    for i in 0..n {
        if !adj[i].is_empty() && seen[i].is_none() {
            let mut todo = VecDeque::new();
            todo.push_back(i);
            seen[i] = Some(false);

            while let Some(from) = todo.pop_front() {
                for &to in &adj[from] {
                    if let Some(flag) = seen[to] {
                        if flag == seen[from].unwrap() {
                            println!("No");
                            return;
                        }
                    } else {
                        todo.push_back(to);
                        seen[to] = Some(!seen[from].unwrap());
                    }
                }
            }
        }
    }

    println!("Yes");
}
