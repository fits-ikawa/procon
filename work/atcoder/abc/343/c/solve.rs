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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let k_max = n.cbrt();

    for i in (1..=k_max).rev() {
        let k = i * i * i;
        let k_str = k.to_string();
        let k_str_rev = k_str.chars().rev().collect::<String>();

        if k_str == k_str_rev {
            println!("{}", k);
            break;
        }
    }
}
