#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        vec: [i32; n],
    }

    debug!(vec.binary_search(&10));
    debug!(vec.binary_search(&3));
    debug!(vec.binary_search(&39));

    debug!(vec.binary_search(&-100));
    debug!(vec.binary_search(&9));
    debug!(vec.binary_search(&100));

    debug!(vec.lower_bound(&9));
    debug!(vec.upper_bound(&20));

    debug!(vec.lower_bound(&40));
    debug!(vec.upper_bound(&-40));

    debug!(vec.lower_bound(&1));
    debug!(vec.lower_bound(&50));

    debug!(vec.lower_bound(&3));
    debug!(vec.upper_bound(&3));
}
