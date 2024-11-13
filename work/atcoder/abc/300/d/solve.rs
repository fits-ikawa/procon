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

    let max_c = (n / 12).sqrt();
    let p = primes(max_c);

    let mut ans = 0;

    // 列挙される素数は最大ケースで 25112 個。
    // その三重ループはふつう計算量として多すぎるが、
    // break で大半を枝刈りできるので十分高速に解ける
    for i in 0..p.len() - 2 {
        for j in i + 1..p.len() - 1 {
            let aab = p[i] * p[i] * p[j];
            if aab > n {
                break;
            }
            for k in j + 1..p.len() {
                if aab.saturating_mul(p[k]).saturating_mul(p[k]) > n {
                    break;
                } else {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
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
