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
        n: usize, d: usize,
        s: Chars,
    }

    let count = s.iter().copied().counts();
    let cookies = count.get(&'@').unwrap_or(&0);

    let mut ans = vec![];
    let mut cur = 0;

    for i in 0..n {
        ans.push(if s[i] == '@' && cur < cookies - d {
            cur += 1;
            '@'
        } else {
            '.'
        });
    }

    println!("{}", ans.iter().join(""));
}
