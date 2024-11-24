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
        n: usize
    }

    let mut nums = (1..=(2 * n + 1)).collect::<HashSet<_>>();

    loop {
        let &takahashi = nums.iter().next().unwrap();
        nums.remove(&takahashi);

        println!("{}", takahashi);

        input_interactive! {
            aoki: usize,
        }
        if aoki == 0 {
            break;
        }
        nums.remove(&aoki);
    }

    println!("{}", 0);
}
