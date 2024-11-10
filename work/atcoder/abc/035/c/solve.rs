#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use proconio::{marker::*, *};
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
        n: usize, q: usize,
        lr: [(usize, usize); q],
    }

    let mut imos = vec![0; n + 1];

    for (l, r) in lr {
        let (l, r) = (l - 1, r - 1);
        imos[l] += 1;
        imos[r + 1] -= 1;
    }

    let ans = imos
        .iter()
        .cumsum::<i64>()
        .take(n)
        .map(|x| if x % 2 == 0 { '0' } else { '1' })
        .collect::<String>();

    println!("{}", ans);
}

fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    let (mut base, mut exp) = (base, exp);
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp /= 2;
    }
    result
}
