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
        lrxy: [(usize, usize, usize, usize); n],
    }

    let mut imos = vec![vec![0_isize; 1010]; 1010];

    for (lx, ly, rx, ry) in lrxy {
        imos[lx][ly] += 1;
        imos[rx][ry] += 1;
        imos[lx][ry] -= 1;
        imos[rx][ly] -= 1;
    }

    for i in 0..1010 {
        for j in 1..1010 {
            imos[i][j] += imos[i][j - 1];
        }
    }

    for j in 0..1010 {
        for i in 1..1010 {
            imos[i][j] += imos[i - 1][j];
        }
    }

    let cnt = imos.into_iter().flatten().counts();
    let ans = (1..=n)
        .map(|i| cnt.get(&(i as isize)).unwrap_or(&0))
        .collect_vec();

    println!("{}", ans.iter().join("\n"));
}
