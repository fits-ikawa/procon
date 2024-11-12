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
        mut s: [Chars; h],
    }

    for i in 0..h {
        for j in 0..w - 1 {
            if s[i][j] == 'T' && s[i][j + 1] == 'T' {
                s[i][j] = 'P';
                s[i][j + 1] = 'C';
            }
        }
    }

    println!(
        "{}",
        s.iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n")
    );
}
