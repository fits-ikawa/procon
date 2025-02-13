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
    // 左右からの累積 GCD
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut gcd_l = vec![0; n];
    let mut gcd_r = vec![0; n];

    for i in 0..n - 1 {
        gcd_l[i + 1] = gcd_l[i].gcd(&a[i]);
        gcd_r[n - i - 2] = gcd_r[n - i - 1].gcd(&a[n - i - 1]);
    }

    let mut ans = 0;

    for i in 0..n {
        ans = ans.max(gcd_l[i].gcd(&gcd_r[i]));
    }

    println!("{}", ans);
}

use ac_library::{Monoid, Segtree};

struct Gcd;

impl Monoid for Gcd {
    type S = usize;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a.gcd(b)
    }
}

#[allow(dead_code)]
fn solve() {
    // セグ木（累積 GCD）
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut seg = Segtree::<Gcd>::new(n);

    for i in 0..n {
        seg.set(i, a[i]);
    }

    let mut ans = 0;

    for i in 0..n {
        ans = ans.max(seg.prod(0..i).gcd(&seg.prod(i + 1..n)));
    }

    println!("{}", ans);
}
