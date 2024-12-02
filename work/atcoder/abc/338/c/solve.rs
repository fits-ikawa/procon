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
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    }

    // A だけ作ったら何人分になるか
    let max_a = izip!(&q, &a)
        .filter_map(|(&qi, &ai)| if ai == 0 { None } else { Some(qi / ai) })
        .min()
        .unwrap();

    let mut ans = 0;

    for i in 0..=max_a {
        let leftover = izip!(&q, &a).map(|(&qi, &ai)| qi - ai * i).collect_vec();
        let num_b = izip!(&leftover, &b)
            .filter_map(|(&qi, &bi)| if bi == 0 { None } else { Some(qi / bi) })
            .min()
            .unwrap();

        ans = ans.max(i + num_b);
    }

    println!("{}", ans);
}
