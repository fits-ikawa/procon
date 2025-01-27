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
    input! {
        n: usize,
        mut s: [Chars; n],
    }

    let s = s
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, i))
        .sorted()
        .collect_vec();

    let mut ans = vec![0; n];

    for i in 0..n - 1 {
        let cnt = izip!(&s[i].0, &s[i + 1].0)
            .take_while(|(a, b)| a == b)
            .count();

        ans[s[i].1] = ans[s[i].1].max(cnt);
        ans[s[i + 1].1] = ans[s[i + 1].1].max(cnt);
    }

    println!("{}", ans.iter().join("\n"));
}
