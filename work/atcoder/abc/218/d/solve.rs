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
        n: usize,
        xy: [(usize, usize); n],
    }

    let xy_set = xy.iter().collect::<HashSet<_>>();

    let mut ans = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            let (ix, iy) = xy[i];
            let (jx, jy) = xy[j];
            // 選んだ 2 点が斜めの関係になっている時、
            // 長方形をつくるための残り 2 点が存在するか確認する
            if ix != jx && iy != jy && xy_set.contains(&(ix, jy)) && xy_set.contains(&(jx, iy)) {
                ans += 1;
            }
        }
    }

    // 同じ長方形は 2 回カウントされている
    println!("{}", ans / 2);
}
