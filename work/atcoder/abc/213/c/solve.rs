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
        _h: usize, _w: usize, n: usize,
        ab: [(usize, usize); n],
    }

    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();

    for &(a, b) in &ab {
        xs.insert(a);
        ys.insert(b);
    }

    // 行方向、列方向それぞれで座標圧縮する
    let xmap = xs
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i + 1))
        .collect::<HashMap<_, _>>();

    let ymap = ys
        .into_iter()
        .enumerate()
        .map(|(i, y)| (y, i + 1))
        .collect::<HashMap<_, _>>();

    for (a, b) in ab {
        println!("{} {}", xmap[&a], ymap[&b]);
    }
}
