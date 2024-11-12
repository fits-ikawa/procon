#![allow(unused_imports)]
use ac_library::{Monoid, Segtree};
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

struct Xor;

impl Monoid for Xor {
    type S = usize;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a ^ b
    }
}

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
        txy: [(usize, usize, usize); q],
    }

    let mut seg_a = Segtree::<Xor>::new(n);
    for i in 0..n {
        seg_a.set(i, a[i]);
    }

    for (t, x, y) in txy {
        match t {
            1 => {
                let ax = seg_a.get(x - 1);
                seg_a.set(x - 1, ax ^ y);
            }
            2 => {
                let sum = seg_a.prod(x - 1..y);
                println!("{}", sum);
            }
            _ => unreachable!(),
        }
    }
}
