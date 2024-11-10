#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering, Reverse};
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
fn main() {
    input! {
        r: usize, c: usize,
        sy: Usize1, sx: Usize1,
        gy: Usize1, gx: Usize1,
        field: [Chars; r],
    }

    let mut seen = vec![vec![None; c]; r];
    let mut todo = VecDeque::new();
    seen[sy][sx] = Some(0);
    todo.push_back((sy, sx));

    while let Some((y, x)) = todo.pop_front() {
        let t = seen[y][x].unwrap();
        for (ny, nx) in udir4(y, x) {
            if field[ny][nx] == '.' && seen[ny][nx].is_none() {
                seen[ny][nx] = Some(t + 1);
                todo.push_back((ny, nx));
            }
        }
    }

    println!("{}", seen[gy][gx].unwrap());
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(y: usize, x: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dy, dx)| {
        let ny = y.wrapping_add(dy);
        let nx = x.wrapping_add(dx);
        (ny, nx)
    })
}
