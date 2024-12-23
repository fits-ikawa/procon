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
        n: usize, m: usize,
        a: [isize; n+1],
        c: [isize; n+m+1],
    }

    let mut b = vec![0; m + 1];

    for i in (0..=m).rev() {
        let mut sum = 0;
        for j in i + 1..=m {
            if i + n >= j {
                sum += b[j] * a[i + n - j];
            }
        }
        b[i] = (c[i + n] - sum) / a[n];
    }

    println!("{}", b.iter().join(" "));
}
