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

    let mut forward = izip!(&s, &t)
        .map(|(&si, &ti)| {
            if si == ti || si == '?' || ti == '?' {
                0
            } else {
                1
            }
        })
        .cumsum::<usize>()
        .collect_vec();

    let mut backward = izip!(s.iter().rev(), t.iter().rev())
        .map(|(&si, &ti)| {
            if si == ti || si == '?' || ti == '?' {
                0
            } else {
                1
            }
        })
        .cumsum::<usize>()
        .collect_vec();

    forward.insert(0, 0);
    backward.insert(0, 0);

    for i in 0..=t.len() {
        if forward[i] == 0 && backward[t.len() - i] == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
