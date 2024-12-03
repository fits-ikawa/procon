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
        __std_itern: usize,
        s: Chars,
        q: usize,
        cd: [(char, char); q],
    }

    let mut table = ('a'..='z').collect_vec();

    for (c, d) in cd {
        for i in 0..table.len() {
            if table[i] == c {
                table[i] = d;
            }
        }
    }

    let ans = s
        .iter()
        .map(|&si| table[(si as u8 - b'a') as usize])
        .collect::<String>();

    println!("{}", ans);
}
