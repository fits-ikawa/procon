#![allow(unused_imports)]
use itertools::{MinMaxResult::*, *};
use maplit::*;
use proconio::{marker::*, *};
use std::collections::*;
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
        mut b: [usize; n-1],
    }

    b.insert(0, 0);

    let mut memo = vec![vec![]; n + 1];

    for i in (1..=n).rev() {
        let salary = match memo[i].iter().minmax() {
            NoElements => 1,
            OneElement(x) => x * 2 + 1,
            MinMax(x, y) => x + y + 1,
        };
        memo[b[i - 1]].push(salary);
    }

    println!("{}", memo[0][0]);
}
