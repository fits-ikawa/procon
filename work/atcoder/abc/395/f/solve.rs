#![allow(clippy::comparison_chain)]
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
        n: usize, x: usize,
        ud: [(usize, usize); n],
    }

    let mut left = 0;
    let mut right = 3000000000;

    while right - left > 1 {
        let mid = (left + right) / 2;

        let check = || {
            let mut range = (0, mid);
            for i in 0..n {
                let (u, d) = ud[i];
                if u + d < mid {
                    return false;
                }

                let grind = u + d - mid;
                let h = (d.saturating_sub(grind), d);

                if h.0 <= range.1 && range.0 <= h.1 {
                    range = (
                        h.0.max(range.0).saturating_sub(x),
                        (h.1.min(range.1) + x).min(mid),
                    );
                } else {
                    return false;
                }
            }

            true
        };

        if check() {
            left = mid;
        } else {
            right = mid;
        }
    }

    let mut ans = 0;

    for (u, d) in ud {
        ans += u + d - left;
    }

    println!("{}", ans);
}
