#![allow(clippy::comparison_chain)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(usize, Usize1); q],
    }

    for (t, k) in tk {
        println!("{}", solve(t, k, &s));
    }
}

fn solve(t: usize, k: usize, s: &[char]) -> char {
    if t == 0 {
        return s[k];
    }

    if k == 0 {
        return rotate(s[0], t);
    }

    rotate(solve(t - 1, k / 2, s), k % 2 + 1)
}

fn rotate(c: char, offset: usize) -> char {
    ['A', 'B', 'C'][(c as usize - 'A' as usize + offset) % 3]
}
