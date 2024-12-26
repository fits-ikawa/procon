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
        a: [usize; n],
    }

    let mut dp_asc = vec![usize::MAX; n];
    let mut asc_len = vec![0; n];

    for i in 0..n {
        let pos = dp_asc.lower_bound(&(a[i]));
        dp_asc[pos] = a[i];
        asc_len[i] = pos + 1;
    }

    let mut dp_desc = vec![usize::MAX; n];
    let mut desc_len = vec![0; n];

    for i in (0..n).rev() {
        let pos = dp_desc.lower_bound(&(a[i]));
        dp_desc[pos] = a[i];
        desc_len[i] = pos + 1;
    }

    let ans = izip!(asc_len, desc_len)
        .map(|(a, d)| a + d - 1)
        .max()
        .unwrap();

    println!("{}", ans);
}
