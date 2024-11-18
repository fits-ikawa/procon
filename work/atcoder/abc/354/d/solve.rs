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
        a: i64, b: i64, c: i64, d: i64,
    }

    // pattern[i][j]
    // (x, y) を左下にした時の数字のパターン
    // i: |y| % 2
    // j: x mod 4
    let pattern = [
        [[2, 1, 0, 1], [1, 0, 1, 2], [0, 1, 2, 1], [1, 2, 1, 0]],
        [[1, 2, 1, 0], [2, 1, 0, 1], [1, 0, 1, 2], [0, 1, 2, 1]],
    ];

    let w = c - a;
    let h = d - b;

    let mut ans = 0;

    // 幅が 4 の倍数の部分は単純計算
    ans += ((w / 4) * 4) * h;

    // 余った部分の下から数えて奇数段の合計
    ans += pattern[(b.abs() % 2) as usize][modulo(a, 4) as usize][0..((w % 4) as usize)]
        .iter()
        .sum::<i64>()
        * (h / 2 + h % 2);

    // 同じく偶数段の合計
    ans += pattern[1 - (b.abs() % 2) as usize][modulo(a, 4) as usize][0..((w % 4) as usize)]
        .iter()
        .sum::<i64>()
        * (h / 2);

    println!("{}", ans);
}

fn modulo(a: i64, m: i64) -> i64 {
    let result = a % m;
    if result < 0 {
        result + m
    } else {
        result
    }
}
