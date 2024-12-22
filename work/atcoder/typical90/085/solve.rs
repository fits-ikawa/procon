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
    // 約数に対する全探索
    input! {
        k: u64,
    }

    let d = divisors(k);
    let n = d.len();

    let mut ans = 0;

    for i in 0..n {
        for j in i..n {
            let ij = d[i].saturating_mul(d[j]);
            if k / ij < d[j] {
                break;
            }
            if k % ij == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

fn divisors(n: u64) -> Vec<u64> {
    let mut asc = vec![];
    let mut desc = vec![];

    for i in 1..=(n as f64).sqrt().floor() as u64 {
        if n % i == 0 {
            asc.push(i);
            if i != n / i {
                desc.push(n / i);
            }
        }
    }

    asc.extend(desc.into_iter().rev());
    asc
}

#[allow(dead_code)]
fn solve() {
    // なんか通っちゃった工夫なし全探索
    input! {
        k: u64,
    }

    let mut ans = 0;

    for i in 1..=k.cbrt() {
        for j in i..=k.sqrt() {
            if k / (i * j) < j {
                break;
            }
            if k % (i * j) == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
