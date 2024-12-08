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
        n: usize, m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort();
    b.sort();
    a.reverse();
    b.reverse();

    let mut ans = 0;

    while !a.is_empty() && !b.is_empty() {
        let ai = a.pop().unwrap();
        if b.last().copied().unwrap() <= ai {
            ans += ai;
            b.pop();
        }
    }

    if !b.is_empty() {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
