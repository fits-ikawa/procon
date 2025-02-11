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
        p: [Usize1; n],
        q: [usize; n],
    }

    let mut q2i = vec![0; n + 1];

    for i in 0..(n) {
        q2i[q[i]] = i;
    }

    let ans = (1..=n).map(|i| q[p[q2i[i]]]).collect_vec();

    println!("{}", ans.iter().join(" "));
}
