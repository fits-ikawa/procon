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
        n: usize,
        ax: Usize1, ay: Usize1,
        bx: Usize1, by: Usize1,
        s: [Chars; n],
    }

    // 拡張 01-BFS
    let mut todo = VecDeque::new();
    let mut seen = vec![vec![[usize::MAX; 4]; n]; n];

    let dx = [!0, 1, !0, 1];
    let dy = [!0, 1, 1, !0];

    for dir in 0..4 {
        todo.push_back((ax, ay, dir, 0));
        seen[ax][ay][dir] = 0;
    }

    while let Some((x, y, dir, cost)) = todo.pop_front() {
        if seen[x][y][dir] < cost {
            continue;
        }

        // そのままの方向に進む（コスト 0）
        let nx = x.wrapping_add(dx[dir]);
        let ny = y.wrapping_add(dy[dir]);
        if nx < n && ny < n && s[nx][ny] == '.' && seen[nx][ny][dir] > cost {
            seen[nx][ny][dir] = cost;
            todo.push_front((nx, ny, dir, cost));
        }

        // 方向転換する（コスト 1）
        for ndir in if dir <= 1 { [2, 3] } else { [0, 1] } {
            if seen[x][y][ndir] > cost + 1 {
                seen[x][y][ndir] = cost + 1;
                todo.push_back((x, y, ndir, cost + 1));
            }
        }
    }

    let ans = seen[bx][by].iter().min().copied().unwrap();

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans + 1);
    }
}
