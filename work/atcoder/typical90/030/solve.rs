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
        n: usize, k: usize,
    }

    // cnt[i]: 整数 i の素因数の種類数
    let mut cnt = vec![0; n + 1];

    for i in 2..=n {
        if cnt[i] == 0 {
            for j in (i..=n).step_by(i) {
                cnt[j] += 1;
            }
        }
    }

    println!("{}", cnt.iter().filter(|&&x| x >= k).count());
}
