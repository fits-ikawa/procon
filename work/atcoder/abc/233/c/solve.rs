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
        n: usize, x: usize,
    }

    let mut l = vec![];
    let mut a = vec![];

    for _ in 0..n {
        input! {
            l_in: usize, a_in: [usize; l_in],
        }

        l.push(l_in);
        a.push(a_in);
    }

    let mut ans = 0;

    for balls in a.into_iter().multi_cartesian_product() {
        if balls.iter().fold(1, |acc, b| b.saturating_mul(acc)) == x {
            ans += 1;
        }
    }

    println!("{}", ans);
}
