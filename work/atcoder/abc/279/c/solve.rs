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
        h: usize, w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }

    let mut st = vec![vec![0; h]; w];
    let mut tt = vec![vec![0; h]; w];

    for i in 0..h {
        for j in 0..w {
            st[j][i] = if s[i][j] == '#' { 1 } else { 0 };
            tt[j][i] = if t[i][j] == '#' { 1 } else { 0 };
        }
    }

    st.sort();
    tt.sort();

    println!("{}", if st == tt { "Yes" } else { "No" });
}
