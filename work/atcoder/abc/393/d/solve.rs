#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let ones = (0..n).filter(|&i| s[i] == '1').collect_vec();
    let center = ones.len() / 2;

    let mut ans = 0;

    for i in 0..ones.len() {
        if i != center {
            let p = ones[center].abs_diff(ones[i]); // 中央までの距離
            let q = (center).abs_diff(i); // 中央までの 1 の数
            ans += (p - 1) - (q - 1);
        }
    }

    println!("{}", ans);
}
