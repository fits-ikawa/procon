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
        n: usize, d: usize,
        a: [usize; n],
    }

    use ac_library::{Max, Segtree};

    // seg[j]
    // j で終わる部分列の長さの最大値
    let mut seg = Segtree::<Max<usize>>::new(500001);

    for i in 0..n {
        let l = (a[i].saturating_sub(d)).max(1);
        let r = (a[i] + d).min(500000);
        seg.set(a[i], seg.prod(l..r + 1) + 1);
    }

    println!("{}", seg.all_prod());
}
