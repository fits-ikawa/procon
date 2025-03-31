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
        n: usize, _x: usize, _y: usize,
        a: [usize; n],
    }

    // x = 2, y = 3 のときの Grundy 数の 1 周期分
    let grundy = [0, 0, 1, 1, 2];

    let ans = a.iter().fold(0, |acc, &ai| acc ^ grundy[ai % 5]);

    println!("{}", if ans > 0 { "First" } else { "Second" });
}
