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
        d: isize,
    }

    let mut ans = isize::MAX;

    for x in 0..=d.sqrt() + 1 {
        if x * x > d {
            ans = ans.min((x * x - d).abs());
        } else {
            let y = (d - x * x).sqrt();
            ans = ans.min((x * x + y * y - d).abs());
            ans = ans.min((x * x + (y + 1) * (y + 1) - d).abs());
        }
    }

    println!("{}", ans);
}
