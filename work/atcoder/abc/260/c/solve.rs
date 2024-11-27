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

enum Jewel {
    Red(usize, usize),
    Blue(usize, usize),
}

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize, x: usize, y: usize,
    }

    use Jewel::*;

    let mut bag = vec![];
    bag.push(Red(n, 1));

    let mut ans = 0;

    while let Some(j) = bag.pop() {
        match j {
            Red(level, amount) => {
                if level >= 2 {
                    bag.push(Red(level - 1, amount));
                    bag.push(Blue(level, amount * x));
                }
            }
            Blue(level, amount) => {
                if level == 1 {
                    ans += amount;
                } else {
                    bag.push(Red(level - 1, amount));
                    bag.push(Blue(level - 1, amount * y));
                }
            }
        }
    }

    println!("{}", ans);
}
