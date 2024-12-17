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
        n: usize, k: usize,
    }

    use ac_library::ModInt1000000007 as Mint;

    let ans = [
        Mint::new(k),
        Mint::new(k - 1),
        Mint::new(k.saturating_sub(2)).pow(n.saturating_sub(2) as u64),
    ]
    .iter()
    .take(n.min(3))
    .product::<Mint>();

    println!("{}", ans);
}
