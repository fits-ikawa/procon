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
        lr: [(isize, isize); n],
    }

    let (min, max) = lr
        .iter()
        .fold((0, 0), |acc, &(l, r)| (acc.0 + l, acc.1 + r));

    if !(min..=max).contains(&0) {
        println!("No");
        return;
    }

    // 全部最低値を選んだ状態から必要な分だけ上げていく
    let mut m = min;
    let mut ans = vec![];

    for (l, r) in lr {
        if m + r - l < 0 {
            ans.push(r);
            m += r - l;
        } else {
            ans.push(l + m.abs());
            m = 0;
        }
    }

    // 検算
    // debug!(ans.iter().sum::<isize>());

    println!("Yes");
    println!("{}", ans.iter().join(" "));
}
