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
        n: usize, a: usize, b: usize,
    }

    let lcm = a.lcm(&b);
    let mut ans = (1 + n) * n / 2;

    ans += lcm * ((1 + n / lcm) * (n / lcm) / 2);
    ans -= a * ((1 + n / a) * (n / a) / 2);
    ans -= b * ((1 + n / b) * (n / b) / 2);

    println!("{}", ans);
}
