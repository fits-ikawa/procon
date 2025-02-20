#![allow(clippy::comparison_chain)]
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
        _n: usize, m: usize,
        uv: [(usize, usize); m],
    }

    let reduced = uv
        .into_iter()
        .filter_map(|(u, v)| {
            if u == v {
                None
            } else if u < v {
                Some((u, v))
            } else {
                Some((v, u))
            }
        })
        .unique()
        .count();

    println!("{}", m - reduced);
}
