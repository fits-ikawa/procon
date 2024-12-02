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
        n: isize,
        a: [Chars; n],
    }

    let a = a
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|aij| (aij as u8 - b'0') as isize)
                .collect_vec()
        })
        .collect_vec();

    let mut ans = vec![];

    for x in 0..n {
        for y in 0..n {
            for &(dx, dy) in &IDIR8 {
                let seq = dfs(x, y, dx, dy, n - 1, 0, n, &a);
                ans.push(seq);
            }
        }
    }

    ans.sort();

    println!("{}", ans.last().unwrap());
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    cur: isize,
    seq: isize,
    n: isize,
    a: &[Vec<isize>],
) -> isize {
    if cur < 0 {
        return seq;
    }

    dfs(
        (x + dx).mod_floor(&n),
        (y + dy).mod_floor(&n),
        dx,
        dy,
        cur - 1,
        seq + a[x as usize][y as usize] * 10_isize.pow(cur as u32),
        n,
        a,
    )
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
