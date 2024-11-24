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

    if s.len() % 2 == 0 {
        // 文字列の長さが偶数
        println!("No");
        return;
    }

    let center = n / 2;

    if s[center] != '/' {
        // 中央が '/' ではない
        println!("No");
        return;
    }

    for i in 0..center {
        if s[i] != '1' || s[n - 1 - i] != '2' {
            // '/' より前はすべて '1'、後ろはすべて '2' の条件を満たさない
            println!("No");
            return;
        }
    }

    // s == ['/'] のケースは上のループが実行されずここに着く
    println!("Yes");
}
