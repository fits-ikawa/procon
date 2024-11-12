#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use regex::Regex;
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
        s: String,
    }

    let parity = s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == 'B' { Some(i % 2) } else { None })
        .collect::<HashSet<_>>();

    let mut valid = parity.len() == 2;

    valid &= Regex::new(r"R.*K.*R").unwrap().is_match(&s);

    println!("{}", if valid { "Yes" } else { "No" });
}
