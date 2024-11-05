#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize, m: usize,
    }

    if m < n * 2 || n * 4 < m {
        // 全員が大人の時の足の数よりも M が少ない
        // または、全員が赤ちゃんの時の足の数よりも M が多い
        // -> 実現できる組み合わせはない
        println!("-1 -1 -1");
        return;
    }

    // 全員が赤ちゃんと考えると足の数は 4*N 本。
    // ここから赤ちゃんをひとり老人に変えると、足の数を -1 していける。
    // 全員が老人になったら（3*N 本）、今度は老人をひとり大人に変えると
    // また足の数を -1 していける。
    // こうして足の数は 4*N から 2*N まで 1 ずつ動かせるので、
    // 最初のガードで弾かれなければ必ず組み合わせが存在する。

    if n * 3 < m {
        // 大人がいないケース
        let a3 = n * 4 - m;
        println!("{} {} {}", 0, a3, n - a3);
    } else {
        // 赤ちゃんがいないケース
        let a2 = n * 3 - m;
        println!("{} {} {}", a2, n - a2, 0);
    }
}
