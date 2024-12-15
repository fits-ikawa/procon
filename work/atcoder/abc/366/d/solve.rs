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
        a: [[[usize; n]; n]; n],
        q: usize,
        lr: [(usize, usize, usize, usize, usize, usize); q],
    }

    // 三次元累積和
    let mut acc = vec![vec![vec![0; n + 1]; n + 1]; n + 1];

    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                acc[x + 1][y + 1][z + 1] =
                    a[x][y][z] + acc[x][y + 1][z + 1] + acc[x + 1][y][z + 1] + acc[x + 1][y + 1][z]
                        - (acc[x][y][z + 1] + acc[x][y + 1][z] + acc[x + 1][y][z])
                        + acc[x][y][z];
            }
        }
    }

    for (lx, rx, ly, ry, lz, rz) in lr {
        let (lx, ly, lz) = (lx - 1, ly - 1, lz - 1);
        let ans = acc[rx][ry][rz] + (acc[lx][ly][rz] + acc[lx][ry][lz] + acc[rx][ly][lz])
            - (acc[lx][ry][rz] + acc[rx][ly][rz] + acc[rx][ry][lz])
            - acc[lx][ly][lz];
        println!("{}", ans);
    }
}
