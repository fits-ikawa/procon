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

    let mut b = vec![];

    for ai in a {
        b.push(ai);
        let mut m = b.len();
        while m > 1 && b[m - 2] == b[m - 1] {
            let k = b.pop().unwrap();
            b.pop();
            b.push(k + 1);
            m = b.len();
        }
    }

    println!("{}", b.len());
}
