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
    // 座標圧縮＋いもす法
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let comp = ab
        .iter()
        .flat_map(|&(a, b)| [a, a + b])
        .sorted()
        .unique()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<BTreeMap<_, _>>();

    let mut imos = vec![0; comp.len()];

    for (a, b) in ab {
        imos[comp[&a]] += 1;
        imos[comp[&(a + b)]] -= 1;
    }

    let imos = imos.iter().cumsum::<isize>().collect_vec();

    // 座標復元
    let resume = comp
        .keys()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    let mut ans = vec![0; n + 1];

    for (m, d) in izip!(imos, resume) {
        ans[m as usize] += d;
    }

    println!("{}", ans[1..].iter().join(" "));
}
