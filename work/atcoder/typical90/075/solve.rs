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
        n: u64,
    }

    println!("{}", prime_factors(n).len().next_power_of_two().ilog2());
}

fn prime_factors(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    }

    let mut factors = vec![];
    let mut n = n;

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    while n % 3 == 0 {
        factors.push(3);
        n /= 3;
    }

    for i in (5..=(n as f64).sqrt().floor() as u64).step_by(6) {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }

        while n % (i + 2) == 0 {
            factors.push(i + 2);
            n /= i + 2;
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}
