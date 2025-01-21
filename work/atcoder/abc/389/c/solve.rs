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
        q: usize,
    }

    let mut acc = vec![0];
    let mut left = 0;

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    l: usize,
                }

                let last = acc.last().unwrap();
                acc.push(last + l);
            }
            2 => {
                left += 1;
            }
            3 => {
                input! {
                    k: usize,
                }

                println!("{}", acc[left + k - 1] - acc[left]);
            }
            _ => unreachable!(),
        }
    }
}
