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
        q: usize,
        tx: [(usize, usize); q],
    }

    let mut deck = VecDeque::new();

    for (t, x) in tx {
        match t {
            1 => {
                deck.push_front(x);
            }
            2 => {
                deck.push_back(x);
            }
            3 => {
                println!("{}", deck[x - 1]);
            }
            _ => unreachable!(),
        }
    }
}
