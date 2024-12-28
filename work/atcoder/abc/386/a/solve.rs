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
        abcd: [usize; 4],
    }

    let cnt = abcd
        .into_iter()
        .counts()
        .values()
        .sorted()
        .copied()
        .collect_vec();

    if cnt.len() == 2 && (cnt[0] == 1 && cnt[1] == 3) || (cnt[0] == 2 && cnt[1] == 2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
