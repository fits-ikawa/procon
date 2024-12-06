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
        a: [usize; n],
    }

    let mut primes = hashset! {};

    for ai in a {
        for p in prime_factors(ai) {
            primes.insert(p);
        }
    }

    let mut check = vec![false; m + 1];

    for p in primes {
        for i in 1..=m / p {
            check[p * i] = true;
        }
    }

    let ans = check
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(i, &ci)| if ci { None } else { Some(i) })
        .collect_vec();

    println!("{}", ans.len());
    println!("{}", ans.iter().join("\n"));
}

fn prime_factors(n: usize) -> Vec<usize> {
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

    for i in (5..=(n as f64).sqrt().floor() as usize).step_by(6) {
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
