#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use std::fmt::Binary;
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
        a: [Usize1; n],
        w: [usize; n],
    }

    let mut b = vec![BinaryHeap::new(); n];

    for i in 0..n {
        b[a[i]].push(Reverse(w[i]));
    }

    let mut ans = 0;

    for mut bi in b {
        while bi.len() >= 2 {
            let Reverse(weight) = bi.pop().unwrap();
            ans += weight;
        }
    }

    println!("{}", ans);
}
