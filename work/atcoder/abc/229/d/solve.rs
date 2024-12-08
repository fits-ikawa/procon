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
        s: Chars,
        k: usize,
    }

    // 尺取り法（'.' が K 個以下の最長区間）
    let mut right = 0;
    let mut count = if s[0] == '.' { 1 } else { 0 };
    let mut ans = 0;

    for left in 0..s.len() {
        while right + 1 < s.len() && count + if s[right + 1] == '.' { 1 } else { 0 } <= k {
            right += 1;
            count += if s[right] == '.' { 1 } else { 0 };
        }

        if count <= k {
            ans = ans.max(right - left + 1);
        }
        count -= if s[left] == '.' { 1 } else { 0 };

        if left == right && right + 1 < s.len() {
            right += 1;
            count += if s[right] == '.' { 1 } else { 0 };
        }
    }

    println!("{}", ans);
}
