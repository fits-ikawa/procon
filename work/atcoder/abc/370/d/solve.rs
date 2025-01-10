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
        h: usize, w: usize, q: usize,
        rc: [(Usize1, Usize1); q],
    }

    let mut row = vec![BTreeSet::from_iter(0..w); h];
    let mut col = vec![BTreeSet::from_iter(0..h); w];

    for (r, c) in rc {
        if row[r].contains(&c) {
            row[r].remove(&c);
            col[c].remove(&r);
        } else {
            if let Some(up) = row[r].range(..c).next_back().copied() {
                row[r].remove(&up);
                col[up].remove(&r);
            }
            if let Some(down) = row[r].range(c..).next().copied() {
                row[r].remove(&down);
                col[down].remove(&r);
            }
            if let Some(left) = col[c].range(..r).next_back().copied() {
                col[c].remove(&left);
                row[left].remove(&c);
            }
            if let Some(right) = col[c].range(r..).next().copied() {
                col[c].remove(&right);
                row[right].remove(&c);
            }
        }
    }

    println!("{}", row.iter().map(|s| s.len()).sum::<usize>());
}
