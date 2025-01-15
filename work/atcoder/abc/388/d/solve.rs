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
        a: [usize; n],
    }

    let mut b = vec![0; n];
    let mut imos = vec![0; n];
    let mut cur = 0;

    for i in 0..n {
        let s = a[i] + cur;
        if s > 0 {
            cur += 1;

            if i + s < n {
                imos[i + s] += 1;
            }
        }
        b[i] = s.saturating_sub(n - i - 1);
        cur -= imos[i];
    }

    println!("{}", b.iter().join(" "));
}
