#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        _n: usize, d: usize, k: usize,
        lr: [(usize, usize); d],
        st: [(usize, usize); k],
    }

    for (s, t) in st {
        let mut cur = s;
        let mut day = 0;
        while cur != t {
            let (l, r) = lr[day];
            if l <= cur && cur <= r {
                // 移動可能なら目的地方向に最大限移動する
                cur = if t <= cur { t.max(l) } else { t.min(r) };
            }
            day += 1;
        }
        println!("{}", day);
    }
}
