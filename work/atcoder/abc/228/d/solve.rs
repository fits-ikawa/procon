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
        q: usize,
        tx: [(usize, usize); q],
    }

    const N: usize = 1048576; // 2**20

    let mut a = vec![-1; N];
    let mut idx = (0..N).collect::<BTreeSet<_>>();

    for (t, x) in tx {
        if t == 1 {
            let h = x % N;

            let iter = idx.range(h..).next();
            let i = if iter.is_some() {
                iter.copied().unwrap()
            } else {
                idx.range(0..).next().copied().unwrap()
            };

            a[i] = x as isize;
            idx.remove(&i);
        } else {
            println!("{}", a[x % N]);
        }
    }
}
