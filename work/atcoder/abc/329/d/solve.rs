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
        n: usize, m: usize,
        a: [Usize1; m],
    }

    let mut vote = vec![0; n];
    let mut winner = 0;
    let mut winner_vote = 0;

    for ai in a {
        vote[ai] += 1;
        winner = if vote[ai] > winner_vote || (vote[ai] == winner_vote && ai < winner) {
            ai
        } else {
            winner
        };
        winner_vote = winner_vote.max(vote[ai]);

        println!("{}", winner + 1);
    }
}
