#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize, a: i32, b: i32,
        sd: [(String, i32); n],
    }

    let mut cur = 0;

    for (s, d) in sd {
        cur += d.clamp(a, b) * if s == "East" { 1 } else { -1 };
    }

    if cur == 0 {
        println!("0");
    } else {
        println!("{} {}", if cur >= 0 { "East" } else { "West" }, cur.abs());
    }
}
