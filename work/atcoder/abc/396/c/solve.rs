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
        n: usize, m: usize,
        mut b: [isize; n],
        mut w: [isize; m],
    }

    b.sort();
    w.sort();

    let mut ans = 0;
    let mut cnt = 0;

    while !b.is_empty() && b.last().copied().unwrap() > 0 {
        ans += b.pop().unwrap();
        cnt += 1;
    }

    for _ in 0..cnt {
        if !w.is_empty() && w.last().copied().unwrap() > 0 {
            ans += w.pop().unwrap();
        }
    }

    while !b.is_empty()
        && !w.is_empty()
        && b.last().copied().unwrap() + w.last().copied().unwrap() >= 0
    {
        ans += b.pop().unwrap();
        ans += w.pop().unwrap();
    }

    println!("{}", ans);
}
