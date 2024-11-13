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
        h: isize, w: isize,
        s: [Chars; h],
    }

    for (x, y) in iproduct!((0..h), (0..w)) {
        for &(dx, dy) in &IDIR8 {
            if check(x, y, dx, dy, 0, h, w, &s) {
                for k in 0..5 {
                    println!("{} {}", x + dx * k + 1, y + dy * k + 1);
                }
                return;
            }
        }
    }
}

const IDIR8: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[allow(clippy::too_many_arguments)]
fn check(
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    i: usize,
    h: isize,
    w: isize,
    s: &[Vec<char>],
) -> bool {
    if i >= 5 {
        return true;
    }
    if x < 0 || x >= h || y < 0 || y >= w {
        return false;
    }

    if s[x as usize][y as usize] == ['s', 'n', 'u', 'k', 'e'][i] {
        check(x + dx, y + dy, dx, dy, i + 1, h, w, s)
    } else {
        false
    }
}
