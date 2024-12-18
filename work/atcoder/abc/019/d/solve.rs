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
fn main() {
    input_interactive! {
        n: usize,
    }

    let mut cost = vec![0; n];

    for i in 1..n {
        println!("? {} {}", 1, i + 1);

        input_interactive! {
            dist : usize,
        }

        cost[i] = dist;
    }

    let farthest = cost
        .into_iter()
        .enumerate()
        .max_by_key(|&(_, x)| x)
        .unwrap()
        .0;

    let mut cost = vec![0; n];

    for i in 0..n {
        if i == farthest {
            continue;
        }
        println!("? {} {}", farthest + 1, i + 1);

        input_interactive! {
            dist : usize,
        }

        cost[i] = dist;
    }

    println!("! {}", cost.iter().max().unwrap());
}
