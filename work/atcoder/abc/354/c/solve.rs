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
        ac: [(usize, usize); n],
    }

    // A が最大のカードは必ず残される。
    // A が高い順に見ていき、それまで見たカードの中で最小の C 以下の
    // C をもつカードのみ残していく
    let aci = ac
        .iter()
        .enumerate()
        .map(|(i, &(a, c))| (a, c, i + 1))
        .sorted()
        .collect_vec();

    let mut min_c = usize::MAX;
    let mut ans = vec![];

    for j in (0..n).rev() {
        let (_, c, i) = aci[j];
        if c <= min_c {
            ans.push(i);
        }
        min_c = min_c.min(c);
    }

    ans.sort();

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
