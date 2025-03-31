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
        a: [usize; n],
        d: usize,
        lr: [(Usize1, Usize1); d],
    }

    let mut max_l = vec![0; n];
    let mut max_r = vec![0; n];

    max_l[0] = a[0];
    max_r[n - 1] = a[n - 1];

    for i in 1..n {
        max_l[i] = max_l[i - 1].max(a[i]);
        max_r[n - 1 - i] = max_r[n - i].max(a[n - 1 - i]);
    }

    for (l, r) in lr {
        println!("{}", max_l[l - 1].max(max_r[r + 1]));
    }
}
