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
        n: usize, q: usize,
        xy: [(isize, isize); n],
        qs: [Usize1; q],
    }

    let mut xy45 = vec![];

    let mut x_min = isize::MAX;
    let mut x_max = isize::MIN;
    let mut y_min = isize::MAX;
    let mut y_max = isize::MIN;

    for (x, y) in xy {
        let rx = x - y;
        let ry = x + y;

        xy45.push((rx, ry));
        x_min = x_min.min(rx);
        x_max = x_max.max(rx);
        y_min = y_min.min(ry);
        y_max = y_max.max(ry);
    }

    for qi in qs {
        let (x, y) = xy45[qi];
        let ans = x
            .abs_diff(x_min)
            .max(x.abs_diff(x_max))
            .max(y.abs_diff(y_min))
            .max(y.abs_diff(y_max));

        println!("{}", ans);
    }
}
