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

use ac_library::ModInt1000000007 as Mint;

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        l: usize, r: usize,
    }

    println!("{}", calc(r) - calc(l - 1));
}

fn calc(n: usize) -> Mint {
    let mut ans = Mint::new(0);

    for i in 1..=19 {
        let base = 10_usize.pow(i);
        let first = Mint::new(10_usize.pow(i - 1));
        if base <= n {
            let last = Mint::new(base - 1);
            ans += (first * i + last * i) * (last - first + 1) / 2;
        } else {
            let last = Mint::new(n);
            ans += (first * i + last * i) * (last - first + 1) / 2;
            break;
        }
    }

    ans
}
