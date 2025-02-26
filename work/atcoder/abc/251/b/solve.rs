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
        n: usize, w: usize,
        a: [usize; n],
    }

    let mut check = vec![false; w + 1];

    for k in 1..=3 {
        for comb in a.iter().combinations(k) {
            let sum = comb.iter().copied().sum::<usize>();

            if sum <= w {
                check[sum] = true;
            }
        }
    }

    println!("{}", check.iter().filter(|&&x| x).count());
}
