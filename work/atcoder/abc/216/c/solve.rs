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

    let mut m = n;
    let mut ans = vec![];

    while m > 0 {
        if m % 2 == 0 {
            m /= 2;
            ans.push('B');
        } else {
            m -= 1;
            ans.push('A');
        }
    }

    // 検算
    // let mut p = 0;
    // for &op in ans.iter().rev() {
    //     if op == 'A' {
    //         p += 1;
    //     } else {
    //         p *= 2;
    //     }
    // }
    // debug!(n, p);

    println!("{}", ans.iter().rev().collect::<String>());
}
