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
    }

    let mut k = vec![];
    let mut a = vec![];

    for _ in 0..n {
        input! {
            ki: usize,
            ai: [usize; ki],
        }

        k.push(ki);
        a.push(ai);
    }

    let set = a
        .iter()
        .map(|ai| ai.iter().copied().collect::<HashSet<_>>())
        .collect_vec();

    let cnt = a.iter().map(|ai| ai.iter().counts()).collect_vec();

    let mut ans = 0.0;

    for comb in (0..n).combinations(2) {
        let (i, j) = (comb[0], comb[1]);
        let common = set[i].intersection(&set[j]);

        let mut p = 0;

        for &x in common {
            p += cnt[i][&x] * cnt[j][&x];
        }

        let subans = p as f64 / (k[i] * k[j]) as f64;
        if subans > ans {
            ans = subans;
        }
    }

    println!("{}", ans);
}
