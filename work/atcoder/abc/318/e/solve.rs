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
        n: usize,
        a: [Usize1; n],
    }

    let mut indices = vec![vec![]; n];

    for i in 0..n {
        indices[a[i]].push(i);
    }

    let mut ans = 0;

    for idx in indices {
        for i in 1..idx.len() {
            let middle = idx[i] - idx[i - 1] - 1;
            ans += i * middle * (idx.len() - i);
        }
    }

    println!("{}", ans);
}
