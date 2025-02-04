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
        s: [Chars; n],
        t: [Chars; m],
    }

    for i in 0..n - m + 1 {
        for j in 0..n - m + 1 {
            let mut valid = true;
            for x in 0..m {
                for y in 0..m {
                    valid &= s[i + x][j + y] == t[x][y];
                }
            }
            if valid {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
