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
        n: usize,
        names: [String; n],
    }

    let count = names.iter().counts();
    println!("{}", count.iter().map(|(k, v)| (v, k)).max().unwrap().1);

    // let count: counter::Counter<_> = names.iter().collect();
    // println!("{}", count.most_common_ordered().first().unwrap().0);
}
