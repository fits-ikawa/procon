#![allow(unused_imports)]
use ac_library::Dsu;
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
        n: usize, d: isize,
        xy: [(isize, isize); n],
    }

    let d = d * d;
    let mut dsu = Dsu::new(n);

    for i in 0..n - 1 {
        for j in i + 1..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            if (x2 - x1).pow(2) + (y2 - y1).pow(2) <= d {
                // 感染距離にいるならグループを統合
                dsu.merge(i, j);
            }
        }
    }

    let mut infected = hashset! {};
    let leader = dsu.leader(0);

    for i in 0..n {
        if dsu.leader(i) == leader {
            infected.insert(i);
        }
    }

    for i in 0..n {
        println!("{}", if infected.contains(&i) { "Yes" } else { "No" });
    }
}
