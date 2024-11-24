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
        s: Chars,
        w: [usize; n],
    }

    let mut x = w.iter().sorted().dedup().copied().collect_vec();
    x.push(usize::MAX);

    let w = w
        .into_iter()
        .enumerate()
        .map(|(i, wi)| (wi, (s[i] as u8 - b'0') as usize))
        .sorted()
        .collect_vec();

    let mut acc = w.iter().map(|&(_, si)| si).cumsum::<usize>().collect_vec();
    acc.insert(0, 0);

    let mut ans = 0;

    for xi in x {
        // xi 未満を子供と判定する
        let pos = w.lower_bound_by_key(&xi, |&(wi, _)| wi);
        let adult = n - pos;
        let f = n - (acc[pos] + adult.abs_diff(acc[n] - acc[pos]));

        ans = ans.max(f);
    }

    println!("{}", ans);
}
