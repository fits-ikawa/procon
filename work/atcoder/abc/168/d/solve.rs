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
        ab: [(usize, usize); m],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a - 1].push(b - 1);
        adj[b - 1].push(a - 1);
    }

    let mut todo = VecDeque::new();
    let mut point = vec![None; n];
    todo.push_back(0);
    point[0] = Some(0);

    while !todo.is_empty() {
        let i = todo.pop_front().unwrap();
        for &j in &adj[i] {
            if point[j].is_none() {
                point[j] = Some(i + 1);
                todo.push_back(j);
            }
        }
    }

    println!("Yes");
    println!("{}", point.iter().skip(1).flatten().join("\n"));
}
