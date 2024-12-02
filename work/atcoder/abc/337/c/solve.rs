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
        a: [isize; n],
    }

    // next[i]
    // i == -1: 先頭の人の番号
    // それ以外: i 番目の人の次の人の番号
    let next = a
        .iter()
        .enumerate()
        .map(|(i, &ai)| (ai, i + 1))
        .collect::<HashMap<_, _>>();

    let mut pos = next[&(-1)];
    let mut ans = vec![pos];

    while let Some(&next_pos) = next.get(&(pos as isize)) {
        ans.push(next_pos);
        pos = next_pos;
    }

    println!("{}", ans.iter().join(" "));
}
