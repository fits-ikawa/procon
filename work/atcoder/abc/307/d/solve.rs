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
        _n: usize,
        s: Chars,
    }

    let mut stack = vec![];
    let mut depth = 0;

    for si in s {
        if si == ')' && depth > 0 {
            while !stack.is_empty() && stack.last().copied().unwrap() != '(' {
                stack.pop();
            }
            stack.pop();
            depth -= 1;
        } else if si == '(' {
            depth += 1;
            stack.push(si);
        } else {
            stack.push(si);
        }
    }

    if !stack.is_empty() {
        println!("{}", stack.iter().collect::<String>());
    }
}
