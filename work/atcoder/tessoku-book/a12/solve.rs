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
        a: [usize; n],
    }

    let mut left = 0;
    let mut right = 2 * 10_usize.pow(18);

    while right - left > 1 {
        let mid = (left + right) / 2;

        let printed = a.iter().map(|&ai| mid / ai).sum::<usize>();

        if printed >= k {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
