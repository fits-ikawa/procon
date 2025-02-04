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
        mut a: [Chars; h],
    }

    let (mut sx, mut sy) = (0, 0);
    let (mut gx, mut gy) = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                sx = i;
                sy = j;
                a[i][j] = '.';
            } else if a[i][j] == 'G' {
                gx = i;
                gy = j;
                a[i][j] = '.';
            }

            let d = match a[i][j] {
                '^' => (!0, 0),
                'v' => (1, 0),
                '<' => (0, !0),
                '>' => (0, 1),
                _ => (0, 0),
            };

            if d != (0, 0) {
                let (dx, dy) = d;
                let mut x = i.wrapping_add(dx);
                let mut y = j.wrapping_add(dy);

                while x < h && y < w && (a[x][y] == '.' || a[x][y] == '!') {
                    a[x][y] = '!';
                    x = x.wrapping_add(dx);
                    y = y.wrapping_add(dy);
                }
            }
        }
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![None; w]; h]; // コストを記録

    todo.push_back((sx, sy));
    seen[sx][sy] = Some(0);

    while let Some((x, y)) = todo.pop_front() {
        for (nx, ny) in udir4(x, y) {
            if nx < h && ny < w && a[nx][ny] == '.' && seen[nx][ny].is_none() {
                seen[nx][ny] = seen[x][y].map(|e| e + 1);
                todo.push_back((nx, ny));
            }
        }
    }

    if let Some(ans) = seen[gx][gy] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dx, dy)| {
        let new_x = x.wrapping_add(dx);
        let new_y = y.wrapping_add(dy);
        (new_x, new_y)
    })
}
