#![allow(unused_imports)]
use counter::Counter;
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(usize, usize); m],
    }

    let mut rel = vec![vec![false; n]; n];

    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
        rel[i][i] = true;
    }

    for (x, y) in edges {
        rel[x - 1][y - 1] = true;
        rel[y - 1][x - 1] = true;
    }

    let mut max_n = 0;

    for i in 1..=n {
        for c in (0..n).combinations(i) {
            if iproduct!(0..c.len(), 0..c.len()).all(|(x, y)| rel[c[x]][c[y]]) {
                max_n = i;
                break;
            }
        }
    }

    println!("{}", max_n);
}
