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

    let mut nums = BTreeMap::new();

    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    x: usize,
                }

                *nums.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize, c: usize,
                }

                if let Some(&now) = nums.get(&x) {
                    if c >= now {
                        nums.remove(&x);
                    } else {
                        nums.insert(x, now - c);
                    }
                }
            }
            3 => {
                let (min, _) = nums.first_key_value().unwrap();
                let (max, _) = nums.last_key_value().unwrap();
                println!("{}", max - min);
            }
            _ => unreachable!(),
        }
    }
}
