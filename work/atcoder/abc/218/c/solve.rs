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
        s: [Chars; n],
        t: [Chars; n],
    }

    let mut s = trim(&s);
    let t = trim(&t);

    let mut ans = false;

    for _ in 0..4 {
        ans |= s == t;
        s = rotate(&s);
    }

    println!("{}", if ans { "Yes" } else { "No" });
}

#[allow(clippy::needless_range_loop)]
fn rotate(p: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut rotated = vec![vec!['.'; p.len()]; p[0].len()];

    for x in 0..p.len() {
        for y in 0..p[0].len() {
            rotated[y][p.len() - x - 1] = p[x][y];
        }
    }

    rotated
}

fn trim(p: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut x_min = usize::MAX;
    let mut x_max = 0;
    let mut y_min = usize::MAX;
    let mut y_max = 0;

    for x in 0..p.len() {
        for y in 0..p[0].len() {
            if p[x][y] == '#' {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }
    }

    let h = x_max - x_min + 1;
    let w = y_max - y_min + 1;

    let mut trimmed = vec![vec!['.'; w]; h];

    for x in 0..p.len() {
        for y in 0..p[0].len() {
            if p[x][y] == '#' {
                trimmed[x - x_min][y - y_min] = '#';
            }
        }
    }

    trimmed
}
