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
        n: usize, _m: usize,
        s: Chars,
        c: [Usize1; n],
    }

    let mut indices = vec![vec![]; n];

    for (i, &ci) in c.iter().enumerate() {
        indices[ci].push(i);
    }

    let mut ans = vec!['.'; n];

    for i in 0..n {
        let index = &indices[c[i]];
        let pos = (index.lower_bound(&i) + 1) % index.len();
        ans[index[pos]] = s[i];
    }

    println!("{}", ans.iter().join(""));
}
