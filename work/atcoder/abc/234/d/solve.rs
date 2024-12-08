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
        n: usize, k: usize,
        p: [usize; n],
    }

    let mut q = p[..k].iter().copied().collect::<BTreeSet<_>>();

    println!("{}", q.first().unwrap());

    for i in k..n {
        if q.first().copied().unwrap() < p[i] {
            q.pop_first();
            q.insert(p[i]);
        }
        println!("{}", q.first().unwrap());
    }
}
