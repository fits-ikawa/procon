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
        s: [char; 3],
        t: [char; 3],
    }

    let impossible =
        vec![s[1], s[0], s[2]] == t || vec![s[2], s[1], s[0]] == t || vec![s[0], s[2], s[1]] == t;

    println!("{}", if impossible { "No" } else { "Yes" });
}
