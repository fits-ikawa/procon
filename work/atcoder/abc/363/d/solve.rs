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
    }

    if n == 1 {
        println!("0");
        return;
    }

    let mut m = n - 1;

    for i in 1.. {
        let base = (i + 1) / 2;
        let cnt = 10_usize.pow(base) - 10_usize.pow(base - 1);

        if m >= cnt {
            m -= cnt;
        } else {
            let left = (10_usize.pow(base - 1) + m - 1).to_string();

            let right = if i % 2 == 0 {
                left.chars().rev().collect::<String>()
            } else {
                left.chars().rev().skip(1).collect::<String>()
            };

            println!("{}{}", left, right);
            break;
        }
    }
}
