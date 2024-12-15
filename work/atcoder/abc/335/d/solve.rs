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

    let mut loong = vec![vec!["".to_owned(); n]; n];

    let turn = hashmap! {
        (0, 1) => (1, 0),
        (1, 0) => (0, !0),
        (0, !0) => (!0, 0),
        (!0, 0) => (0, 1),
    };

    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (0, 1);
    let mut part = 1;

    while !(x == n / 2 && y == n / 2) {
        loong[x][y] = part.to_string();
        part += 1;

        let (nx, ny) = (x.wrapping_add(dx), y.wrapping_add(dy));
        if nx >= n || ny >= n || !loong[nx][ny].is_empty() {
            (dx, dy) = turn[&(dx, dy)];
        }

        (x, y) = (x.wrapping_add(dx), y.wrapping_add(dy));
    }
    loong[n / 2][n / 2] = "T".to_string();

    println!(
        "{}",
        loong.iter().map(|line| line.iter().join(" ")).join("\n")
    );
}
