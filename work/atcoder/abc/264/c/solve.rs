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
        h1: usize, w1: usize,
        a: [[usize; w1]; h1],
        h2: usize, w2: usize,
        b: [[usize; w2]; h2],
    }

    if a == b {
        println!("Yes");
        return;
    }

    for h in (0..h1).combinations(h1 - h2) {
        for w in (0..w1).combinations(w1 - w2) {
            let a_reduced = a
                .iter()
                .enumerate()
                .filter_map(|(i, row)| {
                    if !h.contains(&i) {
                        Some(
                            row.iter()
                                .enumerate()
                                .filter_map(
                                    |(j, &aij)| if !w.contains(&j) { Some(aij) } else { None },
                                )
                                .collect_vec(),
                        )
                    } else {
                        None
                    }
                })
                .collect_vec();

            if a_reduced == b {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
