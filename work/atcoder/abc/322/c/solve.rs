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
        n: usize, m: usize,
        a: [usize; m],
    }

    let mut schedule = vec![false; n + 1];

    for &ai in &a {
        schedule[ai] = true;
    }

    let mut days = vec![0; n + 1];

    for i in (0..=n).rev() {
        days[i] = if schedule[i] { 0 } else { days[i + 1] + 1 }
    }

    for i in 1..=n {
        println!("{}", days[i]);
    }
}
