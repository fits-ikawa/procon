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
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut adj = hashmap! {};

    for (a, b) in ab {
        adj.entry(a).or_insert(vec![]).push(b);
        adj.entry(b).or_insert(vec![]).push(a);
    }

    let mut todo = VecDeque::new();
    todo.push_back(1);
    let mut seen = hashset! {};
    seen.insert(1);

    let mut ans = 1;

    while let Some(from) = todo.pop_front() {
        if let Some(adj) = adj.get(&from) {
            for &to in adj {
                if !seen.contains(&to) {
                    ans = ans.max(to);
                    seen.insert(to);
                    todo.push_back(to);
                }
            }
        }
    }

    println!("{}", ans);
}
