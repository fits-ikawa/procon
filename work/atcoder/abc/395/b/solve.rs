#![allow(clippy::comparison_chain)]
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
    }

    let mut grid = vec![vec!['.'; n]; n];

    for i in 1..=n {
        let j = n + 1 - i;
        if i <= j {
            for x in i..=j {
                for y in i..=j {
                    grid[x - 1][y - 1] = if i % 2 == 1 { '#' } else { '.' };
                }
            }
        }
    }

    println!(
        "{}",
        grid.iter().map(|line| line.iter().join("")).join("\n")
    );
}
