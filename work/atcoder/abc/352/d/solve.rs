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
    // スライド最小/最大値で実装
    input! {
        n: usize, k: usize,
        p: [usize; n],
    }

    let idx = p
        .iter()
        .enumerate()
        .map(|(i, &pi)| (pi, i))
        .sorted()
        .map(|(_, i)| i)
        .collect_vec();

    let mut min_q = VecDeque::new();
    let mut max_q = VecDeque::new();

    for i in 0..k {
        while !min_q.is_empty() && min_q.back().copied().unwrap() > idx[i] {
            min_q.pop_back();
        }
        min_q.push_back(idx[i]);

        while !max_q.is_empty() && max_q.back().copied().unwrap() < idx[i] {
            max_q.pop_back();
        }
        max_q.push_back(idx[i]);
    }

    let mut ans = max_q[0] - min_q[0];

    for i in 1..n - k + 1 {
        if min_q[0] == idx[i - 1] {
            min_q.pop_front();
        }
        while !min_q.is_empty() && min_q.back().copied().unwrap() > idx[i + k - 1] {
            min_q.pop_back();
        }
        min_q.push_back(idx[i + k - 1]);

        if max_q[0] == idx[i - 1] {
            max_q.pop_front();
        }
        while !max_q.is_empty() && max_q.back().copied().unwrap() < idx[i + k - 1] {
            max_q.pop_back();
        }
        max_q.push_back(idx[i + k - 1]);

        ans = ans.min(max_q[0] - min_q[0]);
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // BTreeSet で実装
    input! {
        n: usize, k: usize,
        p: [usize; n],
    }

    let idx = p
        .iter()
        .enumerate()
        .map(|(i, &pi)| (pi, i))
        .sorted()
        .map(|(_, i)| i)
        .collect_vec();

    let mut cur = idx[..k].iter().copied().collect::<BTreeSet<_>>();
    let mut ans = usize::MAX;

    for i in 0..n - k + 1 {
        if i != 0 {
            cur.remove(&idx[i - 1]);
            cur.insert(idx[i + k - 1]);
        }

        let diff = cur
            .first()
            .copied()
            .unwrap()
            .abs_diff(cur.last().copied().unwrap());

        ans = ans.min(diff);
    }

    println!("{}", ans);
}
