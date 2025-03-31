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
        n: usize, x: usize, y: usize,
        a: [usize; n],
    }

    let m = a.iter().max().copied().unwrap();
    let mut grundy = vec![0; m + 1];

    for i in 1..=m {
        let mut transit = [false; 3];

        if i >= x {
            transit[grundy[i - x]] = true;
        }
        if i >= y {
            transit[grundy[i - y]] = true;
        }

        grundy[i] = (0..).find(|&j| !transit[j]).unwrap();
    }

    let ans = a.iter().fold(0, |acc, &ai| acc ^ grundy[ai]);

    println!("{}", if ans > 0 { "First" } else { "Second" });
}
