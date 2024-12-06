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
        n: usize, a: usize, b: usize,
        d: [usize; n],
    }

    // 各予定が一週間の何日目にあたるかが重要なので
    // A+B で割ったあまりに置き換えて重複を消し、昇順にしておく
    let mut d = d
        .into_iter()
        .map(|di| di % (a + b))
        .sorted()
        .dedup()
        .collect_vec();

    // ある予定を基準に未来方向に全予定を数えたとき、
    // 最初の予定と最後の予定が a 日の幅に収まっていれば
    // 全て休日となる可能性がある
    let period = d.len();

    d.extend(d.iter().map(|&di| di + a + b).collect_vec());

    let mut min_seq = usize::MAX;

    for i in 0..period {
        let seq = d[i + period - 1] - d[i] + 1;
        min_seq = min_seq.min(seq);
    }

    let ans = min_seq <= a;

    println!("{}", if ans { "Yes" } else { "No" });
}
