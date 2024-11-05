#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

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
    }

    let mut buf = [0, 0, 1];

    if n <= 3 {
        println!("{}", buf[n - 1]);
        return;
    }

    let mut t = 1;

    for _ in 4..=n {
        t = buf.iter().sum::<usize>() % 10007;
        buf[0] = buf[1];
        buf[1] = buf[2];
        buf[2] = t;
    }

    println!("{}", t);
}
