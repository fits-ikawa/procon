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
        n: usize, p: usize, q: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    for i in 0..n - 4 {
        for j in i + 1..n - 3 {
            for k in j + 1..n - 2 {
                for l in k + 1..n - 1 {
                    for m in l + 1..n {
                        let prod = a[i] * a[j] % p * a[k] % p * a[l] % p * a[m] % p;
                        if prod == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
