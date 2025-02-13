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

use ac_library::{LazySegtree, MapMonoid, Max};

struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<isize>;
    type F = isize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &isize, &x: &isize) -> isize {
        f + x
    }

    fn composition(&f: &isize, &g: &isize) -> isize {
        f + g
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [isize; n],
        b: [usize; m],
    }

    let mut seg = LazySegtree::<MaxAdd>::new(n);

    for i in 0..n {
        seg.set(i, a[i]);
    }

    for bi in b {
        let k = seg.get(bi) as usize;
        seg.set(bi, 0);

        if k / n > 0 {
            seg.apply_range(.., (k / n) as isize);
        }

        if k % n > 0 {
            let r = bi + k % n;
            seg.apply_range(bi + 1..(r + 1).min(n), 1);
            if r >= n {
                seg.apply_range(0..r - n + 1, 1);
            }
        }
    }

    println!("{}", (0..n).map(|i| seg.get(i)).join(" "));
}
