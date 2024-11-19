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
        p1: [Chars; 4],
        p2: [Chars; 4],
        p3: [Chars; 4],
    }

    let mut p1 = trim(&p1);
    let mut p2 = trim(&p2);
    let mut p3 = trim(&p3);

    let board = [['.'; 4]; 4];

    for _ in 0..4 {
        for (x, y) in iproduct!((0..5 - p1.len()), (0..5 - p1[0].len())) {
            let b1 = put(x, y, &p1, &board).unwrap(); // 必ず置ける
            for _ in 0..4 {
                for (x, y) in iproduct!((0..5 - p2.len()), (0..5 - p2[0].len())) {
                    if let Some(b2) = put(x, y, &p2, &b1) {
                        for _ in 0..4 {
                            for (x, y) in iproduct!((0..5 - p3.len()), (0..5 - p3[0].len())) {
                                if let Some(b3) = put(x, y, &p3, &b2) {
                                    // 3 個置けたので埋まったか確認
                                    if is_filled(&b3) {
                                        println!("Yes");
                                        return;
                                    }
                                }
                            }
                            p3 = rotate(&p3);
                        }
                    }
                }
                p2 = rotate(&p2);
            }
        }
        p1 = rotate(&p1);
    }

    println!("No");
}

#[allow(clippy::needless_range_loop)]
fn is_filled(b: &[[char; 4]; 4]) -> bool {
    for x in 0..4 {
        for y in 0..4 {
            if b[x][y] == '.' {
                return false;
            }
        }
    }
    true
}

fn put(x: usize, y: usize, p: &[Vec<char>], b: &[[char; 4]; 4]) -> Option<[[char; 4]; 4]> {
    let mut result = *b;

    for px in 0..p.len() {
        for py in 0..p[0].len() {
            if p[px][py] == '#' {
                if result[x + px][y + py] == '#' {
                    return None;
                } else {
                    result[x + px][y + py] = '#';
                }
            }
        }
    }

    Some(result)
}

#[allow(clippy::needless_range_loop)]
fn rotate(p: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut rotated = vec![vec!['.'; p.len()]; p[0].len()];

    for x in 0..p.len() {
        for y in 0..p[0].len() {
            rotated[y][p.len() - x - 1] = p[x][y];
        }
    }

    rotated
}

fn trim(p: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut x_min = usize::MAX;
    let mut x_max = 0;
    let mut y_min = usize::MAX;
    let mut y_max = 0;

    for x in 0..p.len() {
        for y in 0..p[0].len() {
            if p[x][y] == '#' {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }
    }

    let h = x_max - x_min + 1;
    let w = y_max - y_min + 1;

    let mut trimmed = vec![vec!['.'; w]; h];

    for x in 0..p.len() {
        for y in 0..p[0].len() {
            if p[x][y] == '#' {
                trimmed[x - x_min][y - y_min] = '#';
            }
        }
    }

    trimmed
}
