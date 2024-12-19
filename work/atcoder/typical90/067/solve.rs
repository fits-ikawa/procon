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
        mut n: String, k: usize,
    }

    for _ in 0..k {
        let m = usize::from_str_radix(&n, 8).unwrap();
        n = f(m);
    }

    println!("{}", n);
}

fn f(m: usize) -> String {
    if m == 0 {
        return "0".to_string();
    }

    let mut m = m;
    let mut digits = vec![];
    while m > 0 {
        let r = m % 9;
        digits.push((b'0' + r as u8) as char);
        m /= 9;
    }

    digits
        .into_iter()
        .map(|d| if d == '8' { '5' } else { d })
        .rev()
        .collect::<String>()
}
