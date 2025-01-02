#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = vec![0; n];

    for i in 0..n {
        if a[i] > 0 {
            b[i] = prime_factors(a[i])
                .into_iter()
                .counts()
                .into_iter()
                .fold(1, |acc, (k, v)| if v % 2 == 0 { acc } else { acc * k });
        }
    }

    let cnt = b.into_iter().counts();

    let mut ans = 0;

    for (k, v) in cnt {
        if k == 0 {
            ans += v * (v - 1) / 2 + v * (n - v);
        } else {
            ans += v * (v - 1) / 2;
        }
    }

    println!("{}", ans);
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
