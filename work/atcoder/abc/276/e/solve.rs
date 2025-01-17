#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        c: [Chars; h],
    }

    let (mut sx, mut sy) = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                sx = i;
                sy = j;
            }
        }
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![None; w]; h];

    for (i, (x, y)) in udir4(sx, sy).enumerate() {
        if x < h && y < w && c[x][y] == '.' {
            todo.push_back((x, y));
            seen[x][y] = Some(i);
        }
    }

    let mut ans = false;

    while let Some((x, y)) = todo.pop_front() {
        for (nx, ny) in udir4(x, y) {
            if nx < h && ny < w && c[nx][ny] == '.' {
                if seen[nx][ny].is_none() {
                    seen[nx][ny] = seen[x][y];
                    todo.push_back((nx, ny));
                } else if seen[nx][ny] != seen[x][y] {
                    ans = true;
                }
            }
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dx, dy)| {
        let new_x = x.wrapping_add(dx);
        let new_y = y.wrapping_add(dy);
        (new_x, new_y)
    })
}
