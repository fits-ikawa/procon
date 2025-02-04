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
        d: String,
    }

    let ans = match &d[..] {
        "N" => "S",
        "S" => "N",
        "E" => "W",
        "W" => "E",
        "NE" => "SW",
        "SW" => "NE",
        "NW" => "SE",
        "SE" => "NW",
        _ => unreachable!(),
    };

    println!("{}", ans);
}
