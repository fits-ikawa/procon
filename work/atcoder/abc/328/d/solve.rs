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
        s: Chars,
    }

    let mut stack = vec![];

    for si in s {
        let n = stack.len();
        if si == 'C' && n >= 2 && stack[n - 1] == 'B' && stack[n - 2] == 'A' {
            stack.pop();
            stack.pop();
        } else {
            stack.push(si);
        }
    }

    if !stack.is_empty() {
        println!("{}", stack.iter().collect::<String>());
    }
}
