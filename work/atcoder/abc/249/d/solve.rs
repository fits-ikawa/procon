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
        n: usize,
        a: [u64; n],
    }

    let cnt = a.iter().copied().counts();

    let mut ans = 0;

    for (&ai, &ni) in &cnt {
        for (aj, ak) in divisor_pairs(ai) {
            if let (Some(&nj), Some(&nk)) = (cnt.get(&aj), cnt.get(&ak)) {
                ans += ni * nj * nk;
            }
        }
    }

    println!("{}", ans);
}

fn divisor_pairs(n: u64) -> Vec<(u64, u64)> {
    let mut pairs = vec![];

    for i in 1..=(n as f64).sqrt().floor() as u64 {
        if n % i == 0 {
            if i != n / i {
                pairs.push((i, n / i));
                pairs.push((n / i, i));
            } else {
                pairs.push((i, i));
            }
        }
    }

    pairs
}
