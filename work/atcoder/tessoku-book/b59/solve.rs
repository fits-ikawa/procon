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
        a: [Usize1; n],
    }

    let mut bit = ac_library::FenwickTree::new(n, 0_usize);
    let mut ans = 0;

    for ai in a {
        ans += bit.sum(ai..);
        bit.add(ai, 1);
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    use ac_library::{Additive, Segtree};

    let mut seg = Segtree::<Additive<usize>>::new(n);
    let mut ans = 0;

    for ai in a {
        ans += seg.prod(ai..);
        seg.set(ai, 1);
    }

    println!("{}", ans);
}
