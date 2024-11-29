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
        _n: usize, q: usize,
        tab: [(usize, usize, usize); q],
    }

    let mut ff = hashset! {};

    for (t, a, b) in tab {
        match t {
            1 => {
                ff.insert((a, b));
            }
            2 => {
                ff.remove(&(a, b));
            }
            3 => {
                println!(
                    "{}",
                    if ff.contains(&(a, b)) && ff.contains(&(b, a)) {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            }
            _ => unreachable!(),
        }
    }
}
