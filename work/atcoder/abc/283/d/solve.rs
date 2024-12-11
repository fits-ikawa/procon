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

    let mut b = hashset! {};
    let mut stack = vec![];

    for si in s {
        match si {
            '(' => stack.push('('),
            ')' => {
                while stack.last().copied().unwrap() != '(' {
                    b.remove(&stack.pop().unwrap());
                }
                stack.pop();
            }
            x => {
                if b.contains(&x) {
                    println!("No");
                    return;
                }
                b.insert(x);
                stack.push(x);
            }
        }
    }

    println!("Yes");
}
