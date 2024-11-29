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
        n: isize,
        p: [isize; n],
    }

    // turn[i]
    // i 回テーブルを回したときに喜ぶ人数
    let mut turn = vec![0; n as usize];

    for (i, &p) in p.iter().enumerate() {
        let i = i as isize;
        let front = (p - i).mod_floor(&n);
        for j in -1..=1 {
            turn[(front + j).mod_floor(&n) as usize] += 1;
        }
    }

    println!("{}", turn.iter().max().unwrap());
}
