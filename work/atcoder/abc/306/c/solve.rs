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
        a: [usize; n*3],
    }

    let mut count = vec![0; n + 1];
    let mut f = vec![];

    for (i, ai) in a.into_iter().enumerate() {
        count[ai] += 1;
        if count[ai] == 2 {
            f.push((i, ai));
        }
    }

    f.sort();

    println!("{}", f.iter().map(|(_, x)| x).join(" "));
}
