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
        n: usize, m: usize,
        l: [usize; n],
    }

    let mut left = 0;
    let mut right = 3 * 10_usize.pow(14);

    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut line_wid = 0;
        let mut lines = 1;
        let mut i = 0;

        while i < n {
            if line_wid + l[i] <= mid {
                line_wid += l[i];
                i += 1;
                if line_wid < mid {
                    line_wid += 1;
                }
            } else {
                if line_wid == 0 {
                    // 1 行の幅より長い単語がある
                    break;
                }
                line_wid = 0;
                lines += 1;
            }
        }

        if i == n && lines <= m {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
