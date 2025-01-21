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
        r: u128,
    }

    let r2 = r * 2;
    let mut p = 0;

    for x in (1..=r2).step_by(2) {
        let mut y = (r2 * r2 - x * x).sqrt();
        if y % 2 == 0 {
            y -= 1;
        }
        let z = y / 2;
        p += z;
    }

    let ans = p * 4 + 1;

    println!("{}", ans);
}
