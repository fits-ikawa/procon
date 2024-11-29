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
        a: isize, b: isize, c: isize,
    }

    let ans = if c % 2 == 0 {
        match a.abs().cmp(&b.abs()) {
            Less => '<',
            Equal => '=',
            Greater => '>',
        }
    } else {
        match a.cmp(&b) {
            Less => '<',
            Equal => '=',
            Greater => '>',
        }
    };

    println!("{}", ans);
}
