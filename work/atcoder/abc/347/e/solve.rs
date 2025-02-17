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
        n: usize, q: usize,
        x: [Usize1; q],
    }

    let mut a = vec![0_isize; n];
    let mut set = hashset! {};
    let mut sum = 0;

    for xi in x {
        if set.contains(&xi) {
            a[xi] += sum;
            set.remove(&xi);
        } else {
            a[xi] -= sum;
            set.insert(xi);
        }

        sum += set.len() as isize;
    }

    for i in set {
        a[i] += sum;
    }

    println!("{}", a.iter().join(" "));
}
