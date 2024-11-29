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
    // 賢い全探索（解説 AC）
    input! {
        s: [Chars; 9],
    }

    let mut v = hashset![];

    for (i, j) in iproduct!((0..9), (0..9)) {
        if s[i][j] == '#' {
            v.insert((i as isize, j as isize));
        }
    }

    let mut ans = 0;

    for p in v.iter().permutations(2) {
        let (a, b) = p[0];
        let (c, d) = p[1];

        // p[0], p[1] から時計回りに正方形を成すよう p2, p3 を決める
        let p2 = (c - (d - b), d - (a - c));
        let p3 = (a - (d - b), b - (a - c));

        if v.contains(&p2) && v.contains(&p3) {
            ans += 1;
        }
    }

    // 各正方形は 4 回カウントされている
    println!("{}", ans / 4);
}

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
fn solve() {
    // 根性の全探索
    input! {
        s: [Chars; 9],
    }

    let mut v = vec![];

    for (i, j) in iproduct!((0..9), (0..9)) {
        if s[i][j] == '#' {
            v.push((i as isize, j as isize));
        }
    }

    let ans = v
        .into_iter()
        .combinations(4)
        .filter(|c| {
            c.iter().permutations(4).any(|p| {
                [
                    distsq(p[0], p[1]),
                    distsq(p[1], p[2]),
                    distsq(p[2], p[3]),
                    distsq(p[3], p[0]),
                ]
                .iter()
                .all_equal()
                    && [
                        vec_sub(p[0], p[1]),
                        vec_sub(p[1], p[2]),
                        vec_sub(p[2], p[3]),
                        vec_sub(p[3], p[0]),
                        vec_sub(p[0], p[1]),
                    ]
                    .windows(2)
                    .all(|w| dot(&w[0], &w[1]) == 0)
            })
        })
        .count();

    println!("{}", ans);
}

#[allow(dead_code)]
fn distsq(a: &(isize, isize), b: &(isize, isize)) -> isize {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

#[allow(dead_code)]
fn vec_sub(a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
    (a.0 - b.0, a.1 - b.1)
}

#[allow(dead_code)]
fn dot(a: &(isize, isize), b: &(isize, isize)) -> isize {
    a.0 * b.0 + a.1 * b.1
}
