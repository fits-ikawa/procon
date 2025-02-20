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
        n: usize, q: usize,
        a: [usize; n],
        rx: [(Usize1, usize); q],
    }

    let mut r2ix = hashmap! {};

    for i in 0..q {
        let (r, x) = rx[i];
        let value = r2ix.entry(r).or_insert(vec![]);
        value.push((i, x));
    }

    // 最長増加部分列
    let mut dp = vec![usize::MAX; n];
    let mut ans = vec![0; q];

    for i in 0..n {
        let pos = dp.lower_bound(&a[i]);
        dp[pos] = a[i];

        if let Some(ix) = r2ix.get(&i) {
            for &(j, x) in ix {
                ans[j] = dp.upper_bound(&x);
            }
        }
    }

    println!("{}", ans.iter().join("\n"));
}
