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
        h: usize, w: usize,
        s: [Chars; h],
    }

    let mut x_min = usize::MAX;
    let mut x_max = 0;
    let mut y_min = usize::MAX;
    let mut y_max = 0;

    for x in 0..h {
        for y in 0..w {
            if s[x][y] == '#' {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }
    }

    for i in x_min..=x_max {
        for j in y_min..=y_max {
            if s[i][j] == '.' {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }

    unreachable!();
}
