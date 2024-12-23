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
        a: [usize; n],
    }

    let max_a = a.iter().max().copied().unwrap();

    // cnt[i]: a にある i 以下の要素の個数
    let mut cnt = vec![0; max_a + 1];

    for &ai in &a {
        cnt[ai] += 1;
    }

    for i in 1..=max_a {
        cnt[i] += cnt[i - 1];
    }

    let mut ans = 0_usize;

    for &aj in &a {
        ans += cnt[aj - 1] * (cnt[max_a] - cnt[aj]);
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let cnt = a
        .iter()
        .counts()
        .into_iter()
        .enumerate()
        .map(|(i, (_, v))| (i, v))
        .sorted()
        .collect::<BTreeMap<_, _>>();

    let m = cnt.len();

    let acc = std::iter::once(0)
        .chain(cnt.values().copied())
        .cumsum::<usize>()
        .collect_vec();

    let mut ans = n * (n - 1) * (n - 2) / 6;

    for (&x, &c) in &cnt {
        if c >= 3 {
            // a[i] == a[j] == a[k]
            ans -= c * (c - 1) * (c - 2) / 6;
        }
        if c >= 2 {
            let comb = c * (c - 1) / 2;
            // a[i] == a[j] != a[k]
            ans -= comb * (acc[m] - acc[x + 1]);
            // a[i] != a[j] == a[k]
            ans -= (acc[x] - acc[0]) * comb;
        }
    }

    println!("{}", ans);
}
