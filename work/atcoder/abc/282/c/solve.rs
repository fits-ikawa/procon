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

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut inner = false;
    let mut ans = vec![];

    for si in s {
        let c = match si {
            '"' => {
                inner = !inner;
                '"'
            }
            ',' => {
                if inner {
                    ','
                } else {
                    '.'
                }
            }
            other => other,
        };
        ans.push(c);
    }

    println!("{}", ans.iter().join(""));
}
