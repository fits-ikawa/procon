#![allow(unused_imports)]
use ac_library::ModInt998244353;
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
    use ac_library::ModInt998244353 as Mint;

    input! {
        n: usize,
    }

    let mut ans = Mint::new(0);
    let mut base = 1;

    while n / (base * 10) > 0 {
        base *= 10;
    }

    let mut b = 10;
    while b <= base {
        let end = b - b / 10;
        ans += Mint::new(1 + end) * Mint::new(end) / 2;
        b *= 10;
    }

    let end = Mint::new(n - (base - 1));
    ans += (Mint::new(1) + end) * end / 2;

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // ModInt ライブラリなし
    input! {
        n: usize,
    }

    let m = 998244353;
    let mut ans = 0;
    let mut base = 1;

    while n / (base * 10) > 0 {
        base *= 10;
    }

    let mut b = 10;
    while b <= base {
        let end = b - b / 10;
        ans = (ans + ((1 + end) % m) * (end % m) / 2) % m;
        b *= 10;
    }

    let end = (n - (base - 1)) % m;
    ans = (ans + (1 + end) * end / 2) % m;

    println!("{}", ans);
}
