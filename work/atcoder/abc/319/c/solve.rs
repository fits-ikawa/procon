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
        c: [usize; 9],
    }

    let lines = vec![
        vec![(1, 2), (3, 6), (4, 8)],
        vec![(0, 2), (4, 7)],
        vec![(0, 1), (5, 8), (4, 6)],
        vec![(4, 5), (0, 6)],
        vec![(3, 5), (1, 7), (0, 8), (2, 6)],
        vec![(3, 4), (2, 8)],
        vec![(7, 8), (0, 3), (2, 4)],
        vec![(6, 8), (1, 4)],
        vec![(6, 7), (2, 5), (0, 4)],
    ];

    let mut n = 0;

    for cmb in (0..9).permutations(9) {
        let mut open = [false; 9];
        let mut not_disappoint = true;
        for i in cmb {
            open[i] = true;

            if lines[i]
                .iter()
                .any(|&nums| open[nums.0] && open[nums.1] && c[nums.0] == c[nums.1])
            {
                not_disappoint = false;
                break;
            }
        }
        if not_disappoint {
            n += 1;
        }
    }

    let ans = n as f64 / (1..=9).product::<usize>() as f64;

    println!("{}", ans);
}
