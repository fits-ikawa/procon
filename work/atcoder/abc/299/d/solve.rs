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
fn main() {
    input_interactive! {
        n: usize,
    }

    let mut left = 0;
    let mut right = n - 1;

    while right - left > 1 {
        let mid = (left + right) / 2;

        println!("? {}", mid + 1);

        input_interactive! {
            si: usize,
        }

        if si == 1 {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("! {}", left + 1);
}
