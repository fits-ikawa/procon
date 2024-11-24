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
        h: [isize; 3], w: [isize; 3],
    }

    let mut ans = 0;

    // 条件を満たす配置において、左上 4 マスが決まると残りが一通りに決まる。
    // その 4 マスに入る数字の組み合わせを全探索し、残りを埋められるか調べる
    for (a00, a01, a10, a11) in iproduct!(1..=h[0], 1..=h[0], 1..=h[1], 1..=h[1]) {
        let a02 = h[0] - a00 - a01;
        let a12 = h[1] - a10 - a11;

        if a02 < 1 || a12 < 1 {
            continue;
        }

        let a20 = w[0] - a00 - a10;
        let a21 = w[1] - a01 - a11;

        if a20 < 1 || a21 < 1 {
            continue;
        }

        let a22_a = h[2] - a20 - a21;
        let a22_b = w[2] - a02 - a12;

        if a22_a != a22_b || a22_a < 1 {
            continue;
        }

        ans += 1;
    }

    println!("{}", ans);
}
