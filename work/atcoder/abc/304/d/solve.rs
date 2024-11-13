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
        _w: usize, _h: usize,
        n: usize,
        pq: [(usize, usize); n],
        a_len: usize,
        a: [usize; a_len],
        b_len: usize,
        b: [usize; b_len],
    }

    let mut pieces = hashmap! {};

    for (p, q) in pq {
        // イチゴは左から x 番目、下から y 番目のピースに乗っている
        let x = a.lower_bound(&p);
        let y = b.lower_bound(&q);
        pieces.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut ans_max = usize::MIN;
    let mut ans_min = usize::MAX;

    for &v in pieces.values() {
        ans_max = ans_max.max(v);
        ans_min = ans_min.min(v);
    }

    if pieces.len() < (a_len + 1) * (b_len + 1) {
        ans_min = 0;
    }

    println!("{} {}", ans_min, ans_max);
}
