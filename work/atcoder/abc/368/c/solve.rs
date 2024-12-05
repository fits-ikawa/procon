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
        h: [usize; n],
    }

    let mut t = 0;

    for mut hi in h {
        // t がどの値からでも 3 回攻撃すると 5 ダメージ与えられるので
        // そのぶんをまとめて計算する
        t += (hi / 5) * 3;
        hi %= 5;

        // 端数は攻撃を 1 回ずつシミュレートする
        while hi > 0 {
            t += 1;
            hi = hi.saturating_sub(if t % 3 == 0 { 3 } else { 1 });
        }
    }

    println!("{}", t);
}
