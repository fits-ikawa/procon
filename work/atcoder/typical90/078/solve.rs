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
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut adj = vec![btreeset! {}; n];

    for (a, b) in ab {
        adj[a].insert(b);
        adj[b].insert(a);
    }

    let mut ans = 0;

    for i in 0..n {
        if adj[i].range(..i).count() == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
