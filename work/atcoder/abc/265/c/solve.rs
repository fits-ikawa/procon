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
        g: [Chars; h],
    }

    let mut seen = hashset! {};

    let mut cur = (0, 0);

    loop {
        if seen.contains(&cur) {
            println!("-1");
            return;
        }
        seen.insert(cur);

        let (i, j) = cur;

        match g[i][j] {
            'U' if i != 0 => cur = (i - 1, j),
            'D' if i != h - 1 => cur = (i + 1, j),
            'L' if j != 0 => cur = (i, j - 1),
            'R' if j != w - 1 => cur = (i, j + 1),
            _ => break,
        }
    }

    println!("{} {}", cur.0 + 1, cur.1 + 1);
}
