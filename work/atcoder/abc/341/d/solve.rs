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
        n: usize, m: usize, k: usize,
    }

    let lcm = n.lcm(&m);

    let mut left = 0;
    let mut right = 10_usize.pow(19);

    while right - left > 1 {
        let mid = (left + right) / 2;

        let l = mid / n + mid / m - (mid / lcm) * 2;

        if l >= k {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
