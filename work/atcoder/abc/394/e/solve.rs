#![allow(clippy::comparison_chain)]
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
        c: [Chars; n],
    }

    let mut inset = vec![hashset! {}; n];
    let mut outset = vec![hashset! {}; n];
    let mut inmap = vec![hashmap! {}; n];
    let mut outmap = vec![hashmap! {}; n];

    for i in 0..n {
        for j in 0..n {
            if c[i][j] != '-' {
                inset[j].insert(c[i][j]);
                outset[i].insert(c[i][j]);
                let value = inmap[j].entry(c[i][j]).or_insert(vec![]);
                value.push(i);
                let value = outmap[i].entry(c[i][j]).or_insert(vec![]);
                value.push(j);
            }
        }
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![None; n]; n];

    for i in 0..n {
        todo.push_back((i, i));
        seen[i][i] = Some(0);
    }

    for i in 0..n {
        for j in 0..n {
            if i != j && c[i][j] != '-' {
                todo.push_back((i, j));
                seen[i][j] = Some(1);
            }
        }
    }

    // 両側に回文になるようにパスを伸ばしていく
    while let Some((from, to)) = todo.pop_front() {
        for c in inset[from].intersection(&outset[to]) {
            for (&u, &v) in iproduct!(inmap[from].get(c).unwrap(), outmap[to].get(c).unwrap()) {
                if seen[u][v].is_none() {
                    todo.push_back((u, v));
                    seen[u][v] = seen[from][to].map(|e| e + 2);
                }
            }
        }
    }

    let ans = seen
        .iter()
        .map(|row| row.iter().map(|&x| x.unwrap_or(-1)).collect_vec())
        .collect_vec();

    println!(
        "{}",
        ans.iter().map(|line| line.iter().join(" ")).join("\n")
    );
}
