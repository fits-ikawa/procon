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
        p: [usize; n],
    }

    let cnt = p.iter().copied().counts();
    let mut p2r = [0; 101];
    let mut r = 1;

    let q = p.iter().sorted().dedup().copied().collect_vec();

    for &qi in q.iter().rev() {
        p2r[qi] = r;
        r += cnt[&qi];
    }

    println!("{}", p.iter().map(|&pi| p2r[pi]).join("\n"));
}
