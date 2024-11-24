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
        n: usize, k: usize,
        a: [usize; n],
    }

    let b = a.iter().sorted().copied().collect_vec();

    // 相互に移動できる要素のグループ（a_sub）ごとにソートして
    // B の同じ位置の要素（b_sub）に一致するか確認する
    let ans = (0..k).all(|i| {
        let mut a_sub = vec![];
        let mut b_sub = vec![];
        for j in (i..n).step_by(k) {
            a_sub.push(a[j]);
            b_sub.push(b[j]);
        }
        a_sub.sort();

        a_sub == b_sub
    });

    println!("{}", if ans { "Yes" } else { "No" });
}
