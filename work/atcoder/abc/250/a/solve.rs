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
        r: usize, c: usize,
    }

    let ans = udir4(r, c)
        .map(|(x, y)| {
            if (1..=h).contains(&x) && (1..=w).contains(&y) && r.abs_diff(x) + c.abs_diff(y) == 1 {
                1
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("{}", ans);
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(y: usize, x: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dy, dx)| {
        let new_y = y.wrapping_add(dy);
        let new_x = x.wrapping_add(dx);
        (new_y, new_x)
    })
}
