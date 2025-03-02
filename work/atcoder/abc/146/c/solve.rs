#![allow(clippy::comparison_chain)]
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
        a: usize, b: usize, x: usize,
    }

    let mut left = 0;
    let mut right = 1000000001;

    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut n = mid;
        let mut d = 0;

        while n > 0 {
            d += 1;
            n /= 10;
        }

        if a * mid + b * d <= x {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
