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
        s: String,
    }

    let mut k = 0;

    let mut acc = VecDeque::new();
    acc.push_back('b');

    while acc.len() < n {
        k += 1;
        if k % 3 == 1 {
            acc.push_front('a');
            acc.push_back('c');
        } else if k % 3 == 2 {
            acc.push_front('c');
            acc.push_back('a');
        } else {
            acc.push_front('b');
            acc.push_back('b');
        }
    }

    println!("{}", if acc.iter().join("") == s { k } else { -1 });
}
