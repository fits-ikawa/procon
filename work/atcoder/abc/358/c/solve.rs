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
        n: usize, m: usize,
        s: [Chars; n],
    }

    let s = s
        .into_iter()
        .map(|si| {
            (0..m)
                .map(|j| if si[j] == 'o' { 1 << j } else { 0 })
                .sum::<usize>()
        })
        .collect_vec();

    let mut ans = usize::MAX;

    for i in 1_usize..1 << n {
        let mut bit = 0;
        for j in 0..n {
            if i & 1 << j != 0 {
                bit |= s[j];
            }
        }
        if bit.count_ones() as usize == m {
            ans = ans.min(i.count_ones() as usize);
        }
    }

    println!("{}", ans);
}
