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
    // 解説 AC
    input! {
        n: usize,
        sc: [(usize, usize); n],
    }

    let mut t = BTreeMap::new();

    for (mut s, c) in sc {
        let mut split = 0;

        while s % 2 == 0 {
            split += 1;
            s /= 2;
        }

        let value = t.entry(s).or_insert(0);
        *value += 2_usize.pow(split) * c;
    }

    let mut ans = 0;

    for v in t.values() {
        ans += v.count_ones();
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // シミュレーション
    input! {
        n: usize,
        sc: [(usize, usize); n],
    }

    let mut todo = sc.into_iter().collect::<BTreeMap<_, _>>();
    let mut ans = 0;

    while let Some((s, c)) = todo.pop_first() {
        let nc = c / 2;
        if nc > 0 {
            todo.entry(s * 2).and_modify(|e| *e += nc).or_insert(nc);
        }
        ans += c % 2;
    }

    println!("{}", ans);
}
