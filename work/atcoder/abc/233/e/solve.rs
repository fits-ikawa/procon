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
        mut x: Chars,
    }

    let mut y = x
        .iter()
        .map(|&xi| xi.to_digit(10).unwrap() as usize)
        .cumsum::<usize>()
        .collect_vec();

    y.reverse();

    let mut carry = 0;
    let mut ans = vec![];

    for i in 0..y.len() {
        carry += y[i];
        ans.push(carry % 10);
        carry /= 10;
    }

    while carry > 0 {
        ans.push(carry % 10);
        carry /= 10;
    }

    println!("{}", ans.iter().rev().join(""));
}
