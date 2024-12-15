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
        h: usize, w: usize, k: usize,
        s: [Chars; h],
    }

    let mut ans = solve(&s, h, w, k);

    // 盤面の天地をとってもう一度判定する
    let mut t = vec![vec!['_'; h]; w];
    for i in 0..h {
        for j in 0..w {
            t[j][i] = s[i][j];
        }
    }

    ans = ans.min(solve(&t, w, h, k));

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

#[allow(clippy::needless_range_loop)]
fn solve(s: &[Vec<char>], h: usize, w: usize, k: usize) -> usize {
    let mut ret = usize::MAX;

    for i in 0..h {
        let mut cnt_d = 0;
        let mut cnt_x = 0;
        for j in 0..w {
            match s[i][j] {
                '.' => cnt_d += 1,
                'x' => cnt_x += 1,
                _ => (),
            }

            if j >= k - 1 {
                if cnt_x == 0 {
                    ret = ret.min(cnt_d);
                }
                match s[i][j + 1 - k] {
                    '.' => cnt_d -= 1,
                    'x' => cnt_x -= 1,
                    _ => (),
                }
            }
        }
    }

    ret
}
