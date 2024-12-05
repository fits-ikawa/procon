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
        q: usize,
    }

    let mut bag = hashmap! {};

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: usize,
                }

                *bag.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize,
                }

                bag.entry(x).and_modify(|e| *e -= 1);
                if bag[&x] == 0 {
                    bag.remove(&x);
                }
            }
            3 => {
                println!("{}", bag.len());
            }
            _ => unreachable!(),
        }
    }
}
