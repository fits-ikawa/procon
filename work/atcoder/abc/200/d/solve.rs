#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cand = vec![vec![]; 200];

    for set in (1..=n.min(8)).powerset() {
        if set.is_empty() {
            continue;
        }

        let m = set.iter().map(|&i| a[i - 1]).sum::<usize>() % 200;
        cand[m].push(set);
    }

    for i in 0..200 {
        if cand[i].len() >= 2 {
            println!("Yes");
            println!("{} {}", cand[i][0].len(), cand[i][0].iter().join(" "));
            println!("{} {}", cand[i][1].len(), cand[i][1].iter().join(" "));
            return;
        }
    }

    println!("No");
}
