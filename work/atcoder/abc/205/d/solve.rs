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
        n: usize, q: usize,
        mut a: [usize; n],
        k: [usize; q],
    }

    // num[i] の数字は小さい方から nth[i] 番目
    let mut num = vec![];
    let mut nth = vec![];

    a.insert(0, 0);
    a.push(a.last().unwrap() + 2);

    for i in 1..=n + 1 {
        if a[i - 1] + 1 < a[i] {
            // 連続が途切れたところの
            // それが数字と小さい方から何番目かを記録
            num.push(a[i - 1] + 1);
            nth.push(a[i - 1] + 2 - i);
        }
    }

    for ki in k {
        let pos = nth.upper_bound(&ki) - 1;
        let ans = num[pos] + ki - nth[pos];
        println!("{}", ans);
    }
}
