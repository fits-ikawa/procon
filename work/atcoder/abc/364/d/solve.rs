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
        n: usize, q: usize,
        mut a: [isize; n],
        bk: [(isize, usize); q],
    }

    a.sort();

    for (b, k) in bk {
        let mut left = -1;
        let mut right = 10_isize.pow(9);

        while right - left > 1 {
            let mid = (left + right) / 2;

            let m = a.upper_bound(&(b + mid)) - a.lower_bound(&(b - mid));

            if m >= k {
                right = mid;
            } else {
                left = mid;
            }
        }

        println!("{}", right);
    }
}
