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
    // O(NlogN) 解法
    input! {
        n: usize,
    }

    // div[i]
    // i の約数の個数
    let mut div = vec![0; n];

    for i in 1..n {
        let mut j = 1;
        while i * j < n {
            div[i * j] += 1;
            j += 1;
        }
    }

    let mut ans = 0;

    for ab in 1..=n / 2 {
        let cd = n - ab;

        ans += div[ab] * div[cd];
        if ab != cd {
            ans += div[ab] * div[cd];
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // O(N^1.5) 解法
    input! {
        n: usize,
    }

    let mut ans = 0;

    for ab in 1..=n / 2 {
        let cd = n - ab;

        let ab_comb = count_divisors(ab);
        let cd_comb = count_divisors(cd);

        ans += ab_comb * cd_comb;
        if ab != cd {
            ans += ab_comb * cd_comb;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn count_divisors(n: usize) -> usize {
    let mut count = 0;

    for i in 1..=n.sqrt() {
        if n % i == 0 {
            count += 1;
            if i != n / i {
                count += 1;
            }
        }
    }

    count
}
