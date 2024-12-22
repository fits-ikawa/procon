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
        t: usize,
        a_s: [(usize, usize); t],
    }

    for (a, s) in a_s {
        if s < a * 2 {
            println!("No");
            continue;
        }

        let p = s - a * 2;
        let valid = (0..60).all(|i| !(a >> i & 1 > 0 && p >> i & 1 > 0));

        if valid {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
