#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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

    let mut cnt = [0_usize; 100];

    for ai in a {
        cnt[ai % 100] += 1;
    }

    let ans = (0..=50)
        .map(|i| {
            if i == 0 || i == 50 {
                cnt[i] * cnt[i].saturating_sub(1) / 2
            } else {
                cnt[i] * cnt[100 - i]
            }
        })
        .sum::<usize>();

    println!("{}", ans);
}
