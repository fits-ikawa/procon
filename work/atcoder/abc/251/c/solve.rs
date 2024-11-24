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
        st: [(String, usize); n],
    }

    let mut st = st
        .into_iter()
        .enumerate()
        .map(|(i, (s, t))| (s, t, i))
        .collect_vec();

    st.sort_by(|a, b| a.0.cmp(&b.0));
    st.dedup_by(|a, b| a.0 == b.0);
    st.sort_by(|a, b| match b.1.cmp(&a.1) {
        Equal => a.2.cmp(&b.2),
        other => other,
    });

    println!("{}", st[0].2 + 1);
}
