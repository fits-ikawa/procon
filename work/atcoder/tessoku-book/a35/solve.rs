#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        n: usize,
        a: [usize; n],
    }

    println!("{}", game(1, 1, n, &a));
}

#[memoise::memoise(x <= 2000, y <= 2000)]
fn game(x: usize, y: usize, n: usize, a: &[usize]) -> usize {
    if x == n {
        return a[y - 1];
    }

    let l = game(x + 1, y, n, a);
    let r = game(x + 1, y + 1, n, a);

    if x % 2 == 1 {
        l.max(r)
    } else {
        l.min(r)
    }
}
