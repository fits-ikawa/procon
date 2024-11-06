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
        n: usize, k: usize,
        t: [[usize; k]; n],
    }

    for mut i in 0..k.pow(n as u32) {
        let mut check = 0;
        for item in &t {
            check ^= item[i % k];
            i /= k;
        }
        if check == 0 {
            println!("Found");
            return;
        }
    }

    println!("Nothing");
}
