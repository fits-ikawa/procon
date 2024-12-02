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
        h: isize, w: isize, _n: usize,
        t: Chars,
        s: [Chars; h],
    }

    let path = t
        .iter()
        .map(|&c| match c {
            'L' => (0, -1),
            'R' => (0, 1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => unreachable!(),
        })
        .collect_vec();

    let mut ans = 0;

    for i in 1..h - 1 {
        'next: for j in 1..w - 1 {
            if s[i as usize][j as usize] == '.' {
                let (mut x, mut y) = (i, j);
                for &(dx, dy) in &path {
                    x += dx;
                    y += dy;
                    if s[x as usize][y as usize] != '.' {
                        continue 'next;
                    }
                }
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
