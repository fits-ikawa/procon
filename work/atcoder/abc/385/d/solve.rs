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
        n: usize, m: usize, sx: isize, sy: isize,
        xy: [(isize, isize); n],
        dc: [(char, isize); m],
    }

    let mut col = btreemap! {};
    let mut row = btreemap! {};

    for (x, y) in xy {
        let colset = col.entry(x).or_insert(btreeset! {});
        let rowset = row.entry(y).or_insert(btreeset! {});
        (*colset).insert(y);
        (*rowset).insert(x);
    }

    let (mut x, mut y) = (sx, sy);
    let mut house = hashset! {};

    for (d, c) in dc {
        let (nx, ny) = match d {
            'U' => (x, y + c),
            'D' => (x, y - c),
            'L' => (x - c, y),
            'R' => (x + c, y),
            _ => unreachable!(),
        };

        let mut visited = vec![];

        if nx == x {
            // 縦移動
            let min = y.min(ny);
            let max = y.max(ny);
            if let Some(set) = col.get(&x) {
                for &z in set.range(min..=max) {
                    visited.push((x, z));
                }
            }
        } else if ny == y {
            // 横移動
            let min = x.min(nx);
            let max = x.max(nx);
            if let Some(set) = row.get(&y) {
                for &z in set.range(min..=max) {
                    visited.push((z, y));
                }
            }
        }

        for (vx, vy) in visited {
            house.insert((vx, vy));
            col.get_mut(&vx).unwrap().remove(&vy);
            row.get_mut(&vy).unwrap().remove(&vx);
        }

        x = nx;
        y = ny;
    }

    println!("{} {} {}", x, y, house.len());
}
