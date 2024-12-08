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
        n: usize, q: usize,
    }

    let mut not_called = (1..=n).rev().collect_vec();
    let mut called = btreeset! {};

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                called.insert(not_called.pop().unwrap());
            }
            2 => {
                input! {
                    x: usize,
                }
                called.remove(&x);
            }
            3 => {
                println!("{}", called.first().unwrap());
            }
            _ => unreachable!(),
        }
    }
}
