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
        n: usize, m: usize,
        mut a: [usize; n],
    }

    if a.iter().sum::<usize>() <= m {
        println!("infinite");
        return;
    }

    let mut left = 0;
    let mut right = m * 2;

    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut sum = 0;
        for &ai in &a {
            sum += ai.min(mid);
        }

        if sum <= m {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
