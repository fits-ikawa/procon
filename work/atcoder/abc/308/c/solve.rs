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
        n: usize,
        ab: [(u128, u128); n],
    }

    let mut abi = ab.into_iter().enumerate().collect_vec();

    abi.sort_by(|x, y| {
        let (xi, (xa, xb)) = x;
        let (yi, (ya, yb)) = y;
        match (ya * (xa + xb)).cmp(&(xa * (ya + yb))) {
            Equal => xi.cmp(yi),
            other => other,
        }
    });

    println!("{}", abi.iter().map(|(i, _)| i + 1).join(" "));
}
