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
        p: [usize; n],
        q: [usize; n],
    }

    let mut p_pos = 0;
    let mut q_pos = 0;

    let mut a = (1..=n).collect_vec();
    let mut i = 1_usize;

    loop {
        if a == p {
            p_pos = i;
        }
        if a == q {
            q_pos = i;
        }

        i += 1;

        if !a.next_permutation() {
            break;
        }
    }

    println!("{}", p_pos.abs_diff(q_pos));
}
