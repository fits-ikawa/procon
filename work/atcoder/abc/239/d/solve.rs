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
        a: u64, b: u64, c: u64, d: u64,
    }

    let p = primes(200).into_iter().collect::<HashSet<_>>();

    for takahashi in a..=b {
        if (c..=d).all(|aoki| !p.contains(&(takahashi + aoki))) {
            println!("Takahashi");
            return;
        }
    }

    println!("Aoki");
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

fn primes(n: u64) -> Vec<u64> {
    let table = primes_table(n);
    (2..=n).filter(|&i| table[i as usize]).collect()
}
