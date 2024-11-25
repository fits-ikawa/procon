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
        n: usize, q: usize,
        a: [usize; n],
        xk: [(usize, usize); q],
    }

    let mut count = hashmap! {};
    let mut cache = hashmap! {};

    for (i, &ai) in a.iter().enumerate() {
        let &mut c = count.entry(ai).and_modify(|e| *e += 1).or_insert(1);
        cache.insert((ai, c), i + 1);
    }

    for pair in xk {
        if let Some(nth) = cache.get(&pair) {
            println!("{}", nth);
        } else {
            println!("-1");
        }
    }
}
