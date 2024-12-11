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
        n: usize, m: usize, d: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort();
    b.sort();

    let mut i = n - 1;
    let mut j = m - 1;

    loop {
        if a[i].abs_diff(b[j]) <= d {
            println!("{}", a[i] + b[j]);
            return;
        }
        if a[i] > b[j] {
            if i == 0 {
                break;
            }
            i -= 1;
        } else {
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }

    println!("-1");
}
