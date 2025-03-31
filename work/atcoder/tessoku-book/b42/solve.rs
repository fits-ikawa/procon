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
        ab: [(isize, isize); n],
    }

    let mut ans = 0;

    for front in [1, -1] {
        for back in [1, -1] {
            let mut sum_a = 0;
            let mut sum_b = 0;

            for i in 0..n {
                let (a, b) = ab[i];

                if a * front + b * back > 0 {
                    sum_a += a;
                    sum_b += b;
                }
            }
            ans = ans.max(sum_a.abs() + sum_b.abs());
        }
    }

    println!("{}", ans);
}
