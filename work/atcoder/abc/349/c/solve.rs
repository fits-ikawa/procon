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
        t: Chars,
    }

    let mut j = 0;
    let mut count = 0;

    for i in 0..s.len() {
        if count < 3 && s[i].to_ascii_uppercase() == t[j] {
            count += 1;
            j += 1;
        }
    }

    let ans = count == 3 || count == 2 && t[2] == 'X';

    println!("{}", if ans { "Yes" } else { "No" });
}
