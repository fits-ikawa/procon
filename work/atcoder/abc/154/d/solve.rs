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
        n: usize, k: usize,
        p: [usize; n],
    }

    let e = p
        .iter()
        .map(|&pi| (((1 + pi) * pi) as f64 / 2.0) / pi as f64)
        .collect_vec();

    let acc = std::iter::once(0.0).chain(e).cumsum::<f64>().collect_vec();
    let mut ans = 0.0;

    for i in 0..n - k + 1 {
        let x = acc[i + k] - acc[i];

        if x > ans {
            ans = x;
        }
    }

    println!("{}", ans);
}
