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
        c: [Chars; h],
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![None; w]; h];
    todo.push_back((0, 0));
    seen[0][0] = Some(1);

    while let Some((x, y)) = todo.pop_front() {
        for (nx, ny) in [(x, y + 1), (x + 1, y)] {
            if nx < h && ny < w && c[nx][ny] == '.' && seen[nx][ny].is_none() {
                seen[nx][ny] = seen[x][y].map(|x| x + 1);
                todo.push_back((nx, ny));
            }
        }
    }

    let ans = seen
        .iter()
        .flat_map(|row| row.iter().flatten())
        .max()
        .unwrap();

    println!("{}", ans);
}
