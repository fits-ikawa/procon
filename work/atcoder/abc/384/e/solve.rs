#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use std::ops::Mul;
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
        h: usize, w: usize, x: f64,
        p: Usize1, q: Usize1,
        s: [[usize; w]; h],
    }

    let mut todo = BinaryHeap::new();
    let mut seen = vec![vec![false; w]; h];
    seen[p][q] = true;

    let mut tak_lv = s[p][q];

    for (nx, ny) in udir4(p, q) {
        if nx < h && ny < w {
            seen[nx][ny] = true;
            todo.push((Reverse(s[nx][ny]), (nx, ny)));
        }
    }

    while let Some((Reverse(slv), (sx, sy))) = todo.pop() {
        if slv as f64 >= (tak_lv as f64) / x {
            break;
        }

        tak_lv += slv;

        for (nx, ny) in udir4(sx, sy) {
            if nx < h && ny < w && !seen[nx][ny] {
                seen[nx][ny] = true;
                todo.push((Reverse(s[nx][ny]), (nx, ny)));
            }
        }
    }

    println!("{}", tak_lv);
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dx, dy)| {
        let new_x = x.wrapping_add(dx);
        let new_y = y.wrapping_add(dy);
        (new_x, new_y)
    })
}
