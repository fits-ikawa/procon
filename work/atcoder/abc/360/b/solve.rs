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
        s: Chars, t: Chars,
    }

    for w in 1..s.len() {
        for c in 1..=w {
            let mut substr = vec![];
            let mut i = c;
            while i <= s.len() {
                substr.push(s[i - 1]);
                i += w;
            }
            if substr == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
