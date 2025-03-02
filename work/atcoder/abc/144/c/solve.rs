#![allow(clippy::comparison_chain)]
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
    input! {
        n: u64,
    }

    let d = divisors(n);
    let d_rev = d.iter().rev().copied().collect_vec();

    let ans = izip!(d, d_rev).map(|(x, y)| x - 1 + y - 1).min().unwrap();

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
