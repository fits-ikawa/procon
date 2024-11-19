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
    }

    let mut a: Vec<Vec<_>> = vec![];

    for i in 0..n {
        let mut ai = vec![];
        for j in 0..=i {
            if j == 0 || j == i {
                ai.push(1);
            } else {
                ai.push(a[i - 1][j - 1] + a[i - 1][j]);
            }
        }
        a.push(ai);
    }

    println!("{}", a.iter().map(|row| row.iter().join(" ")).join("\n"));
}
