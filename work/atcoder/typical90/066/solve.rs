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
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut bit = ac_library::FenwickTree::new(101, 0.0);
    let mut ans = 0.0;

    for i in 0..n {
        let (l, r) = lr[i];
        let m = (r - l + 1) as f64;

        for j in l..=r {
            ans += bit.sum(j + 1..) / m;
        }

        for j in l..=r {
            bit.add(j, 1.0 / m);
        }
    }

    println!("{}", ans);
}
