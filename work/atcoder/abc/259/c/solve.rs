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
        s: Chars,
        t: Chars,
    }

    let s = s.into_iter().dedup_with_count().collect_vec();
    let t = t.into_iter().dedup_with_count().collect_vec();

    let ans = s.len() == t.len()
        && izip!(s, t).all(|((s_count, s_char), (t_count, t_char))| {
            s_char == t_char && (s_count == t_count || (s_count >= 2 && s_count < t_count))
        });

    println!("{}", if ans { "Yes" } else { "No" });
}
