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
        w: isize, n: usize,
        lrv: [(isize, isize, isize); n],
    }

    use ac_library::{Max, Segtree};

    // seg.get(j)
    // 料理を選んで作り香辛料を j 消費したときの最大価値
    let mut seg = Segtree::<Max<_>>::from(vec![isize::MIN; w as usize + 1]);
    seg.set(0, 0);

    for (l, r, v) in lrv {
        for j in (0..=w).rev() {
            let l2 = (j - r).max(0) as usize;
            let r2 = (j - l + 1).max(0) as usize;
            let j = j as usize;
            seg.set(j, seg.get(j).max(seg.prod(l2..r2) + v));
        }
    }

    let ans = seg.get(w as usize);

    if ans > 0 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
