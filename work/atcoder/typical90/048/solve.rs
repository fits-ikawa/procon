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
    // ソートで解く
    input! {
        n: usize, k: usize,
        ab: [(usize, usize); n],
    }

    let ans = ab
        .iter()
        .flat_map(|&(a, b)| [a - b, b])
        .sorted()
        .rev()
        .take(k)
        .sum::<usize>();

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // 優先度付きキューで解く
    input! {
        n: usize, k: usize,
        ab: [(usize, usize); n],
    }

    let mut heap = BinaryHeap::new();

    for (i, &(_, b)) in ab.iter().enumerate() {
        heap.push((b, Some(i)));
    }

    let mut ans = 0;

    for _ in 0..k {
        if let Some((score, i)) = heap.pop() {
            ans += score;
            if let Some(i) = i {
                heap.push((ab[i].0 - ab[i].1, None));
            }
        } else {
            break;
        }
    }

    println!("{}", ans);
}
