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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut uf = ac_library::Dsu::new(a.iter().max().copied().unwrap() + 1);

    for i in 0..n / 2 {
        if a[i] != a[n - 1 - i] {
            uf.merge(a[i], a[n - 1 - i]);
        }
    }

    let ans = uf.groups().iter().map(|g| g.len() - 1).sum::<usize>();

    println!("{}", ans);
}
