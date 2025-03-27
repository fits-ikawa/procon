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
        n: usize,
        x: [Usize1; n],
        c: [usize; n],
    }

    let mut cost = hashmap! {};

    for i in 0..n {
        cost.insert((i, x[i]), c[i]);
    }

    let mut seen = vec![false; n];
    let mut ans = 0;

    for i in 0..n {
        if !seen[i] {
            let mut path = vec![];

            dfs(i, &mut path, &mut seen, &x);

            let last = path.last().copied().unwrap();
            let pos = path.iter().position(|&v| v == last).unwrap();

            if pos != path.len() - 1 {
                ans += path[pos..]
                    .iter()
                    .tuple_windows()
                    .map(|(&a, &b)| cost[&(a, b)])
                    .min()
                    .unwrap();
            }
        }
    }

    println!("{}", ans);
}

fn dfs(v: usize, path: &mut Vec<usize>, seen: &mut [bool], x: &[usize]) {
    path.push(v);

    if seen[v] {
        return;
    }

    seen[v] = true;

    dfs(x[v], path, seen, x);
}
