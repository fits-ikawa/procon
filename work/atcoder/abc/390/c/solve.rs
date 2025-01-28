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
        h: usize, w: usize,
        s: [Chars; h],
    }

    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                min_x = min_x.min(i);
                max_x = max_x.max(i);
                min_y = min_y.min(j);
                max_y = max_y.max(j);
            }
        }
    }

    let mut cnt = 0;

    for i in min_x..=max_x {
        for j in min_y..=max_y {
            if s[i][j] == '#' || s[i][j] == '?' {
                cnt += 1;
            }
        }
    }

    println!(
        "{}",
        if cnt == (max_x - min_x + 1) * (max_y - min_y + 1) {
            "Yes"
        } else {
            "No"
        }
    );
}
