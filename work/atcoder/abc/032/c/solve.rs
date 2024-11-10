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
        n: usize, k: usize,
        mut s: [usize; n],
    }

    if s.contains(&0) {
        println!("{}", n);
        return;
    }

    let mut right = 0;
    let mut mult = 1;
    let mut ans = 0;

    for left in 0..n {
        while right < n && mult * s[right] <= k {
            mult *= s[right];
            right += 1;
        }
        ans = ans.max(right - left);

        if left == right {
            right += 1;
        } else {
            mult /= s[left];
        }
    }

    println!("{}", ans);
}
