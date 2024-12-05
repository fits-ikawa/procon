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

    let mut smaller = vec![];
    let mut larger = vec![];

    for (i, (&si, &ti)) in izip!(&s, &t).enumerate() {
        match ti.cmp(&si) {
            Less => smaller.push(i),
            Greater => larger.push(i),
            _ => (),
        }
    }

    let mut s = s;
    let mut ans = vec![];

    for &i in smaller.iter().chain(larger.iter().rev()) {
        s[i] = t[i];
        ans.push(s.iter().collect::<String>());
    }

    println!("{}", ans.len());
    if !ans.is_empty() {
        println!("{}", ans.iter().join("\n"));
    }
}
