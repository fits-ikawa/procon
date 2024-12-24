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
        n: u64,
    }

    let mut ans = 0;

    for i in 1..=n {
        let mut max_sq = 0;
        for j in 1.. {
            if j * j > i {
                break;
            }
            if i % (j * j) == 0 {
                max_sq = j * j;
            }
        }

        for j in 1.. {
            if (i / max_sq) * j * j > n {
                break;
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}
