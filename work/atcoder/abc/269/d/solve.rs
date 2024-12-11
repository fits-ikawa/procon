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
        xy: [(isize, isize); n],
    }

    let mut uf = ac_library::Dsu::new(n);

    for i in 0..n - 1 {
        for j in i + 1..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];

            for (dx, dy) in [(0, 1), (1, 1), (1, 0), (0, -1), (-1, -1), (-1, 0)] {
                if x1 + dx == x2 && y1 + dy == y2 {
                    uf.merge(i, j);
                    break;
                }
            }
        }
    }

    let ans = (0..n).map(|i| uf.leader(i)).unique().count();

    println!("{}", ans);
}
