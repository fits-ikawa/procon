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
        abxy: [(Usize1, Usize1, isize, isize); m],
    }

    let mut adj = vec![vec![]; n];

    for (a, b, x, y) in abxy {
        adj[a].push((b, x, y));
        adj[b].push((a, -x, -y));
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![None; n]; // 位置を記録

    todo.push_back((0, 0, 0));
    seen[0] = Some((0, 0));

    while let Some((from, x, y)) = todo.pop_front() {
        for &(to, dx, dy) in &adj[from] {
            if seen[to].is_none() {
                seen[to] = Some((x + dx, y + dy));
                todo.push_back((to, x + dx, y + dy));
            }
        }
    }

    for p in seen {
        if let Some((x, y)) = p {
            println!("{} {}", x, y);
        } else {
            println!("undecidable");
        }
    }
}
