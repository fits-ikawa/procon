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
        n: usize,
        mut a: [usize; n+1],
        mut b: [usize; n],
    }

    let sum = a.iter().sum::<usize>();

    for i in 0..n {
        let new_ai = a[i].saturating_sub(b[i]);
        b[i] = b[i].saturating_sub(a[i]);
        a[i] = new_ai;

        a[i + 1] = a[i + 1].saturating_sub(b[i]);
    }

    println!("{}", sum - a.iter().sum::<usize>());
}
