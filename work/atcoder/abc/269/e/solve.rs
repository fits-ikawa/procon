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

fn main() {
    input_interactive! {
        n: usize,
    }

    let mut left = 1;
    let mut right = n + 1;

    while right - left > 1 {
        let mid = (left + right) / 2;

        println!("? {} {} {} {}", 1, n, mid, n);

        input_interactive! {
            t: usize,
        }

        if t < n - mid + 1 {
            left = mid;
        } else {
            right = mid;
        }
    }

    let mut top = 1;
    let mut bottom = n + 1;

    while bottom - top > 1 {
        let mid = (top + bottom) / 2;

        println!("? {} {} {} {}", mid, n, 1, n);

        input_interactive! {
            t: usize,
        }

        if t < n - mid + 1 {
            top = mid;
        } else {
            bottom = mid;
        }
    }

    println!("! {} {}", top, left);
}
