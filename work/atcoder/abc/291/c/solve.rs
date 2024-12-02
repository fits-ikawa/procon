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
        _n: usize,
        c: Chars,
    }

    let mut cur = (0, 0);
    let mut seen = hashset![];
    seen.insert(cur);

    for ci in c {
        let (dx, dy) = match ci {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };

        let next = (cur.0 + dx, cur.1 + dy);
        if seen.contains(&next) {
            println!("Yes");
            return;
        }
        seen.insert(next);
        cur = next;
    }

    println!("No");
}
