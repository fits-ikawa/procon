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
        mut a: usize, mut b: usize,
    }

    let mut ans = 0;

    while a != b {
        if a > b {
            ans += a / b;
            a %= b;
            if a == 0 {
                ans -= 1;
                a = b;
            }
        } else {
            ans += b / a;
            b %= a;
            if b == 0 {
                ans -= 1;
                b = a;
            }
        }
    }

    println!("{}", ans);
}
