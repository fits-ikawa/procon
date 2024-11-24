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
        n: isize, q: usize,
        s: Chars,
        tx: [(usize, isize); q],
    }

    let mut start = 0;

    for (t, x) in tx {
        match t {
            1 => start = modulo(start - x, n),
            2 => println!("{}", s[((start + x - 1) % n) as usize]),
            _ => unreachable!(),
        }
    }
}

fn modulo(a: isize, m: isize) -> isize {
    let result = a % m;
    if result < 0 {
        result + m
    } else {
        result
    }
}
