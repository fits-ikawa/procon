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
        n: usize,
    }

    let p = primes((n / 4).sqrt());
    let m = p.len();
    let n_sqrt = n.sqrt();

    let mut ans = 0;

    // p^2*q^2 を探索
    for i in 0..m.saturating_sub(1) {
        if p[i] * p[i] >= n_sqrt {
            break;
        }
        for j in i + 1..m {
            if p[i] * p[i] * p[j] * p[j] <= n {
                ans += 1;
            } else {
                break;
            }
        }
    }

    // p^8 を探索
    for i in 0..m.min(13) {
        if p[i].pow(8) <= n {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn primes_table(n: usize) -> Vec<bool> {
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

fn primes(n: usize) -> Vec<usize> {
    let table = primes_table(n);
    (2..=n).filter(|&i| table[i]).collect()
}
