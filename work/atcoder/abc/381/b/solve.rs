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
    }

    if s.len() % 2 != 0 {
        // 長さが奇数
        println!("No");
        return;
    }

    let chunks = s.chunks(2).collect_vec();
    let mut chars = hashset! {};

    for &c in &chunks {
        if c[0] != c[1] {
            // ペアになってない
            println!("No");
            return;
        }
        chars.insert(c[0]);
    }

    if chunks.len() == chars.len() {
        println!("Yes");
    } else {
        // 2 個より多く現れる文字がある
        println!("No");
    }
}
