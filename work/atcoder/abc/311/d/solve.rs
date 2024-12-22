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
    // 止まるマスを頂点に BFS
    input! {
        n: usize, m: usize,
        s: [Chars; n],
    }

    let mut todo = VecDeque::new();
    let mut stopped = vec![vec![false; m]; n];
    let mut visited = hashset! {};

    for &dir in &UDIR4 {
        todo.push_back(((1_usize, 1_usize), dir));
    }
    stopped[1][1] = true;
    visited.insert((1, 1));

    while let Some(((mut x, mut y), (dx, dy))) = todo.pop_front() {
        while s[x.wrapping_add(dx)][y.wrapping_add(dy)] == '.' {
            x = x.wrapping_add(dx);
            y = y.wrapping_add(dy);
            visited.insert((x, y));
        }

        if !stopped[x][y] {
            for &dir in &UDIR4 {
                stopped[x][y] = true;
                todo.push_back(((x, y), dir));
            }
        }
    }

    println!("{}", visited.len());
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

#[allow(dead_code)]
fn solve() {
    // 全ての移動マスを頂点に BFS
    input! {
        n: usize, m: usize,
        s: [Chars; n],
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![false; m]; n]; // 壁にぶつかって止まった個所のみ記録
    let mut visited = hashset! {};

    for &dir in &UDIR4 {
        todo.push_back(((1_usize, 1_usize), dir));
    }
    seen[1][1] = true;

    while let Some(((x, y), (dx, dy))) = todo.pop_front() {
        visited.insert((x, y));

        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);

        if s[nx][ny] == '.' {
            let mx = nx.wrapping_add(dx);
            let my = ny.wrapping_add(dy);

            if s[mx][my] == '#' && !seen[nx][ny] {
                for &dir in &UDIR4 {
                    seen[nx][ny] = true;
                    todo.push_back(((nx, ny), dir));
                }
            } else {
                todo.push_back(((nx, ny), (dx, dy)));
            }
        }
    }

    println!("{}", visited.len());
}
