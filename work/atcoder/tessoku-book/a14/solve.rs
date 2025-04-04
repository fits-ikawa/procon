#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        n: usize, k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }

    let ab = iproduct!(a, b)
        .map(|(ai, bi)| ai + bi)
        .unique()
        .collect_vec();

    let cd = iproduct!(c, d)
        .map(|(ci, di)| ci + di)
        .sorted()
        .dedup()
        .collect_vec();

    for x in ab {
        if x <= k && cd.binary_search(&(k - x)).is_ok() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
