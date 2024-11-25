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
        _n: usize, a: usize, b: usize,
        p: usize, q: usize, r: usize, s:usize,
    }

    // 操作によって座標 (b, a) を通る
    //   y =  x + c1
    //   y = -x + c2
    // の 2 直線が引かれる。
    // y = a, x = b として c1, c2 を求める
    let c1 = a - b;
    let c2 = a + b;

    for y in p..=q {
        // 描画範囲の中で直線に乗る点を # とする
        let line = (r..=s)
            .map(|x| if y == x + c1 || y == c2 - x { '#' } else { '.' })
            .collect::<String>();

        println!("{}", line);
    }
}
