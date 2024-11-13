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
        h: usize, w: usize,
        mut a: [Chars; h],
        mut b: [Chars; h],
    }

    for (i, j) in iproduct!((0..h), (0..w)) {
        // (i, j) -> A 側の比較開始地点
        let mut ans = true;
        for (bx, by) in iproduct!((0..h), (0..w)) {
            let ax = (i + bx) % h;
            let ay = (j + by) % w;
            ans &= a[ax][ay] == b[bx][by];
        }
        if ans {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
