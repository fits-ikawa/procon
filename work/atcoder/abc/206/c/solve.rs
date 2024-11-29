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
        a: [usize; n],
    }

    let ans = n * (n - 1) / 2
        - a.into_iter()
            .counts()
            .into_iter()
            .filter_map(|(_, v)| {
                if v >= 2 {
                    Some(combination(v, 2))
                } else {
                    None
                }
            })
            .sum::<usize>();

    println!("{}", ans);
}

fn combination(n: usize, r: usize) -> usize {
    assert!(n >= r);
    let r = r.min(n - r);
    let mut result = 1;
    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    result
}
