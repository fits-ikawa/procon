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
        h: usize, w: usize, d: isize,
        s: [Chars; h],
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![None; w]; h]; // コストを記録

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
                todo.push_back((i as isize, j as isize));
                seen[i][j] = Some(0);
            }
        }
    }

    while let Some((x, y)) = todo.pop_front() {
        for (nx, ny) in idir4(x, y) {
            if 0 <= nx
                && nx < h as isize
                && 0 <= ny
                && ny < w as isize
                && seen[nx as usize][ny as usize].is_none()
                && s[nx as usize][ny as usize] != '#'
            {
                seen[nx as usize][ny as usize] = seen[x as usize][y as usize].map(|x| x + 1);
                todo.push_back((nx, ny));
            }
        }
    }

    let ans = seen
        .into_iter()
        .flat_map(|row| row.into_iter().flatten())
        .filter(|&x| x <= d)
        .count();

    println!("{}", ans);
}

const IDIR4: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn idir4(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    IDIR4.iter().map(move |&(dx, dy)| (x + dx, y + dy))
}
