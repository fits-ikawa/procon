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
        board: [[char; 4]; 4],
    }

    let rotated: Vec<Vec<char>> = board
        .into_iter()
        .rev()
        .map(|row| row.into_iter().rev().collect())
        .collect();

    for row in rotated {
        println!("{}", row.iter().join(" "));
    }
}
