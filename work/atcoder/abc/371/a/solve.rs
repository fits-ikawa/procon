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
        sab: char, sac: char, sbc: char,
    }

    let flip = hashmap! {
        '<' => '>',
        '>' => '<'
    };

    let compare = hashmap! {
        ('A', 'B') => sab,
        ('B', 'A') => flip[&sab],
        ('A', 'C') => sac,
        ('C', 'A') => flip[&sac],
        ('B', 'C') => sbc,
        ('C', 'B') => flip[&sbc],
    };

    for p in ['A', 'B', 'C'].into_iter().permutations(3) {
        if compare[&(p[0], p[1])] == '>' && compare[&(p[1], p[2])] == '>' {
            println!("{}", p[1]);
            return;
        }
    }
}
