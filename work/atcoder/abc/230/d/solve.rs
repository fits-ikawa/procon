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
    // 解説 AC
    input! {
        n: usize, d: isize,
        mut lr: [(isize, isize); n],
    }

    lr.sort_by_key(|&(_, r)| r);

    let mut last = isize::MIN;
    let mut ans = 0;

    for (l, r) in lr {
        if last + d - 1 < l {
            last = r;
            ans += 1;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, d: usize,
        lr: [(usize, usize); n],
    }

    let mut heap = BinaryHeap::new();

    for (i, (l, r)) in lr.into_iter().enumerate() {
        heap.push((Reverse(l * 2), i, true));
        heap.push((Reverse((r + d - 1) * 2 + 1), i, false));
    }

    let mut overlap = btreeset! {};
    let mut ans = 0;

    while let Some((Reverse(x), i, is_left)) = heap.pop() {
        if is_left {
            overlap.insert(i);
        } else if overlap.contains(&i) {
            ans += 1;
            overlap.clear();
        }
    }

    println!("{}", ans);
}
