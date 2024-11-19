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
        n: u32,
    }

    let size = 3_usize.pow(n);

    let mut carpet = vec![vec!['.'; size]; size];

    for x in 0..size {
        for y in 0..size {
            if color(x, y, n) {
                carpet[x][y] = '#';
            }
        }
    }

    println!(
        "{}",
        carpet.iter().map(|row| row.iter().join("")).join("\n")
    );
}

fn color(x: usize, y: usize, k: u32) -> bool {
    if k == 0 {
        return true;
    }

    let subsize = 3_usize.pow(k - 1);

    if x / subsize == 1 && y / subsize == 1 {
        return false;
    }

    color(x % subsize, y % subsize, k - 1)
}
