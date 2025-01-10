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
        a: [Chars; h],
        n: usize,
        rce: [(Usize1, Usize1, usize); n],
    }

    let (mut sx, mut sy, mut tx, mut ty) = (0, 0, 0, 0);

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                sx = i;
                sy = j;
            } else if a[i][j] == 'T' {
                tx = i;
                ty = j;
            }
        }
    }

    let mut b = vec![vec![0; w]; h];

    for (r, c, e) in rce {
        b[r][c] = e;
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![None; w]; h]; // そのマスに入った時のエネルギーを記録

    todo.push_back((sx, sy, 0));
    seen[sx][sy] = Some(0);

    while let Some((x, y, e)) = todo.pop_front() {
        if b[x][y] > e {
            todo.push_front((x, y, b[x][y]));
            b[x][y] = 0;
        } else if e > 0 {
            for (nx, ny) in udir4(x, y) {
                if nx < h
                    && ny < w
                    && a[nx][ny] != '#'
                    && (seen[nx][ny].is_none() || seen[nx][ny].unwrap() < e - 1)
                {
                    seen[nx][ny] = Some(e - 1);
                    todo.push_back((nx, ny, e - 1));
                }
            }
        }
    }

    println!("{}", if seen[tx][ty].is_some() { "Yes" } else { "No" });
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dx, dy)| {
        let new_x = x.wrapping_add(dx);
        let new_y = y.wrapping_add(dy);
        (new_x, new_y)
    })
}
