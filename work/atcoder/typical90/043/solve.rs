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
        rs: Usize1, cs: Usize1,
        rt: Usize1, ct: Usize1,
        s: [Chars; h],
    }

    // 拡張 01-BFS
    let mut todo = VecDeque::new();
    let mut cost = vec![vec![vec![usize::MAX; 4]; w]; h]; // 曲がった回数を記録

    for i in 0..4 {
        todo.push_back((rs, cs, i, 0));
    }

    while let Some((x, y, z, c)) = todo.pop_front() {
        if c > cost[x][y][z] {
            continue;
        }

        let (dx, dy) = UDIR4[z];
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);
        if nx < h && ny < w && s[nx][ny] == '.' && cost[nx][ny][z] > c {
            cost[nx][ny][z] = c;
            todo.push_front((nx, ny, z, c));
        }

        for i in 0..4 {
            if cost[x][y][i] > c + 1 {
                cost[x][y][i] = c + 1;
                todo.push_back((x, y, i, (c + 1)));
            }
        }
    }

    println!("{}", cost[rt][ct].iter().min().unwrap());
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];
