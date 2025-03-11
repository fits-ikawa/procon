#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        s: [Chars; h],
    }

    // 01-BFS

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![usize::MAX; w]; h];

    todo.push_back((0, 0, 0));
    seen[0][0] = 0;

    while let Some((x, y, cost)) = todo.pop_front() {
        if seen[x][y] < cost {
            continue;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if (0..h as isize).contains(&nx) && (0..w as isize).contains(&ny) {
                let nx = nx as usize;
                let ny = ny as usize;
                if s[nx][ny] == '.' && seen[nx][ny] > cost {
                    seen[nx][ny] = cost;
                    todo.push_front((nx, ny, cost));
                }
            }
        }

        for dx in -2..=2_isize {
            for dy in -2..=2_isize {
                if dx.abs() + dy.abs() == 4 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if (0..h as isize).contains(&nx) && (0..w as isize).contains(&ny) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if seen[nx][ny] > cost + 1 {
                        seen[nx][ny] = cost + 1;
                        todo.push_back((nx, ny, cost + 1))
                    }
                }
            }
        }
    }

    println!("{}", seen[h - 1][w - 1]);
}
