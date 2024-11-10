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
        mut a: [Usize1; n],
    }

    let mut indices = vec![0; n];
    for i in 0..n {
        indices[a[i]] = i;
    }

    let mut history = vec![];

    for i in 0..n {
        if a[i] != i {
            let j = indices[i];
            indices[a[i]] = j;
            indices[i] = i;
            a.swap(i, j);
            history.push([i + 1, j + 1]);
        }
    }

    println!("{}", history.len());
    if !history.is_empty() {
        println!("{}", history.iter().map(|x| x.iter().join(" ")).join("\n"));
    }
}
