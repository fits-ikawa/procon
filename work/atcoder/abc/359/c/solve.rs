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
        sx: usize, sy: usize,
        tx: usize, ty: usize,
    }

    let sx = sx - if (sx + sy) % 2 == 0 { 0 } else { 1 };
    let tx = tx - if (tx + ty) % 2 == 0 { 0 } else { 1 };

    // 縦移動は 1 段ごとにコスト 1 が必ずかかる
    let mut ans = sy.abs_diff(ty);

    // 縦移動後、横移動が残っていたらその分のコストを追加
    if sx.abs_diff(tx) > ans {
        ans += (sx.abs_diff(tx) - ans) / 2;
    }

    println!("{}", ans);
}
