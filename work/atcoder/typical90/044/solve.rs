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
        n: isize, q: usize,
        mut a: [usize; n],
        txy: [(usize, isize, isize); q],
    }

    let mut offset = 0;

    for (t, x, y) in txy {
        match t {
            1 => {
                let i = (x - 1 - offset).mod_floor(&n) as usize;
                let j = (y - 1 - offset).mod_floor(&n) as usize;
                a.swap(i, j);
            }
            2 => {
                offset = (offset + 1) % n;
            }
            3 => {
                let i = (x - 1 - offset).mod_floor(&n) as usize;
                println!("{}", a[i]);
            }
            _ => unreachable!(),
        }
    }
}
