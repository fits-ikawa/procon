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
        n: usize, mut s: Chars, q: usize,
        txc: [(usize, usize, char); q]
    }

    let last2 = txc.iter().rposition(|(t, _, _)| *t == 2);
    let last3 = txc.iter().rposition(|(t, _, _)| *t == 3);
    let op_pos = last2.max(last3);

    for (i, (t, x, c)) in txc.into_iter().enumerate() {
        if t == 1 {
            s[x - 1] = c;
        } else if Some(i) == op_pos {
            if t == 2 {
                s = s.into_iter().map(|c| c.to_ascii_lowercase()).collect_vec();
            } else {
                s = s.into_iter().map(|c| c.to_ascii_uppercase()).collect_vec();
            }
        }
    }

    println!("{}", s.iter().collect::<String>());
}
