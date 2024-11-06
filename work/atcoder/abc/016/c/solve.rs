#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
use superslice::*;

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

    let mut fs = vec![vec![]; n];

    for (a, b) in edges {
        fs[a - 1].push(b - 1);
        fs[b - 1].push(a - 1);
    }

    for i in 0..n {
        let ffs: HashSet<_> = fs[i]
            .iter()
            .flat_map(|&j| fs[j].iter().filter(|&&k| i != k && !fs[k].contains(&i)))
            .collect();

        println!("{}", ffs.len());
    }
}
