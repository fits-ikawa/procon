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

    let prime_list = primes((n / 2).cbrt());

    if prime_list.len() < 2 {
        println!("0");
        return;
    }

    // 素数をキーに検索できる素数の個数の累積和
    let prime_count = prime_list
        .iter()
        .scan(0, |acc, &p| {
            *acc += 1;
            Some((p, *acc))
        })
        .collect::<HashMap<_, _>>();

    let mut ans = 0;

    for p_i in 0..prime_list.len() - 1 {
        let p = prime_list[p_i];
        // p に対する最大の q を求める
        let max_qi = prime_list.upper_bound(&(n / p).cbrt()) - 1;
        let max_q = prime_list[max_qi];
        if max_q <= p {
            break;
        }
        // (p, max_q] となる素数の数を記録
        ans += prime_count[&max_q] - prime_count[&p];
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
