#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use proconio::{marker::*, *};
use source::once;
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        q: usize,
        lr: [(usize, usize); q],
    }

    let is_prime = primes_table(100000);

    let acc: Vec<i32> = (0..=100000)
        .map(|i| {
            if is_prime[i] && is_prime[(i + 1) / 2] {
                1
            } else {
                0
            }
        })
        .cumsum()
        .collect();

    for (l, r) in lr {
        println!("{}", acc[r] - acc[l - 1]);
    }
}

fn primes_table(n: u64) -> Vec<bool> {
    let n = n as usize;

    if n < 2 {
        return vec![false; n + 1];
    }

    let mut table = vec![true; n + 1];
    table[0] = false;
    table[1] = false;

    for i in 2..=(n as f64).sqrt().floor() as usize {
        if table[i] {
            for j in ((i * i)..=n).step_by(i) {
                table[j] = false;
            }
        }
    }

    table
}
