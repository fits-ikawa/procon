#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        n: usize, m: usize,
        s: [isize; n-1],
        x: [isize; m],
    }

    let mut a0 = vec![0; n];

    for i in 0..n - 1 {
        a0[i + 1] = s[i] - a0[i];
    }

    let mut table = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            table[i][j] = (x[j] - a0[i]) * if i % 2 == 0 { 1 } else { -1 };
        }
    }

    let ans = table
        .into_iter()
        .flatten()
        .counts()
        .values()
        .max()
        .copied()
        .unwrap();

    println!("{}", ans);
}
