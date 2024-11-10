#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use proconio::{marker::*, *};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.push(0);

    let mut right = 0;
    let mut ans = 0;

    for left in 0..n {
        while a[right] < a[right + 1] {
            right += 1;
        }
        ans += right - left + 1;

        if left == right {
            right += 1;
        }
    }

    println!("{}", ans);
}
