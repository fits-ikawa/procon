#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering, Reverse};
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
fn main() {
    input! {
        n: Chars,
    }

    let a = n[0];
    let b = n[1];
    let c = n[2];

    println!(
        "{} {}",
        [b, c, a].iter().collect::<String>(),
        [c, a, b].iter().collect::<String>()
    );
}
