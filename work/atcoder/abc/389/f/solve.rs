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
        n: usize,
        lr: [(isize, isize); n],
        q: usize,
        x: [isize; q],
    }

    let x = x
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, xi)| xi)
        .collect_vec();

    let mut seg = LazySegtree::<MaxAdd>::new(q);

    for i in 0..q {
        seg.set(i, x[i].1);
    }

    for (l, r) in lr {
        let l2 = seg.max_right(0, |x| x < l);
        let r2 = seg.max_right(0, |x| x <= r);
        seg.apply_range(l2..r2, 1);
    }

    let mut ans = vec![0; q];

    for i in 0..q {
        ans[x[i].0] = seg.get(i);
    }

    println!("{}", ans.iter().join("\n"));
}
