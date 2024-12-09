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
        n: usize, p: usize, q: usize, r: usize,
        a: [usize; n],
    }

    let acc = std::iter::once(0)
        .chain(a.iter().copied())
        .cumsum::<usize>()
        .collect_vec();

    for x in 0..n {
        if let Ok(y) = acc.binary_search(&(acc[x] + p)) {
            if let Ok(z) = acc.binary_search(&(acc[y] + q)) {
                if acc.binary_search(&(acc[z] + r)).is_ok() {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
