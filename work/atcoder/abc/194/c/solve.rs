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
        n: usize,
        a: [isize; n],
    }

    let mut nums = hashmap! {};

    for ai in a {
        nums.entry(ai).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut ans = 0;

    for ((xi, xn), (yi, yn)) in iproduct!(nums.iter(), nums.iter()) {
        let mut m = xn * yn;
        if xi == yi {
            m -= 1;
        }
        ans += (xi - yi).pow(2) * m;
    }

    ans /= 2;

    println!("{}", ans);
}
