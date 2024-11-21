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
    }

    let mut ans = usize::MAX;

    for a in 1..=m.sqrt() + 1 {
        let b = (m + a - 1) / a;

        if a <= n && b <= n {
            ans = ans.min(a * b);
        }
    }

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
