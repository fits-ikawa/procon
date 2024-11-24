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
        s: Chars,
    }

    let mut ans = 0;

    for i in 0..n {
        if s[i] == '/' {
            // i から左右方向に確認開始
            ans = ans.max(1); // "/" も 11/22 文字列の条件を満たす
            let mut j = 1;
            loop {
                if i < j || i + j >= n || s[i - j] != '1' || s[i + j] != '2' {
                    // 範囲外か 11/22 文字列の条件が崩れたら i についての確認は終了
                    break;
                }
                ans = ans.max(j * 2 + 1);
                j += 1;
            }
        }
    }

    println!("{}", ans);
}
