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
        ab: [(Usize1, Usize1); m],
        q: usize,
        xk: [(Usize1, usize); q],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    for (x, k) in xk {
        let mut todo = VecDeque::new();
        let mut seen = hashmap! {};
        let mut ans = x + 1;

        if k > 0 {
            todo.push_back(x);
            seen.insert(x, 0);
        }

        while let Some(from) = todo.pop_front() {
            for &to in &adj[from] {
                if !seen.contains_key(&to) {
                    seen.insert(to, seen[&from] + 1);
                    ans += to + 1;

                    if seen[&to] < k {
                        todo.push_back(to);
                    }
                }
            }
        }

        println!("{}", ans);
    }
}
