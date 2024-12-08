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
        h: usize, w: usize, d: isize,
        s: [Chars; h],
    }

    let mut floor = vec![];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                floor.push((i as isize, j as isize));
            }
        }
    }

    let mut ans = 0;

    for comb in floor.iter().combinations(2) {
        let mut humid = hashset! {};
        for &(x, y) in comb {
            for i in -10..=10 {
                for j in -10..=10 {
                    let (fx, fy) = (i + x, j + y);
                    if i.abs() + j.abs() <= d
                        && 0 <= fx
                        && fx < h as isize
                        && 0 <= fy
                        && fy < w as isize
                        && s[fx as usize][fy as usize] == '.'
                    {
                        humid.insert((fx, fy));
                    }
                }
            }
        }
        ans = ans.max(humid.len());
    }

    println!("{}", ans);
}
