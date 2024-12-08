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
        n: usize, k: isize,
        a: [isize; n],
    }

    let acc = std::iter::once(0).chain(a).cumsum::<isize>().collect_vec();
    let mut acc_map = hashmap! {};

    for i in 1..=n {
        (*acc_map.entry(acc[i]).or_insert(btreeset! {})).insert(i);
    }

    let acc_map = acc_map
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect_vec()))
        .collect::<HashMap<_, _>>();

    let mut ans = 0;

    for i in 0..n {
        if let Some(idx) = acc_map.get(&(acc[i] + k)) {
            ans += idx.len() - idx.upper_bound(&i);
        }
    }

    println!("{}", ans);
}
