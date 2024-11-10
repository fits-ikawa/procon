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
        ab: [(isize, isize); m],
    }

    let mut reachable = hashset! {};

    for (i, j) in ab {
        reachable.insert((i, j));
        for (di, dj) in [
            (2, 1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
        ] {
            let ni = i + di;
            let nj = j + dj;
            if 1 <= ni && ni <= n as isize && 1 <= nj && nj <= n as isize {
                reachable.insert((i + di, j + dj));
            }
        }
    }

    println!("{}", n * n - reachable.len());
}
