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
        n: usize, m: usize,
        ps: [(Usize1, String); m],
    }

    let mut ac = vec![false; n];
    let mut wa = vec![0; n];
    let mut ac_cnt = 0;

    for (p, s) in ps {
        if s == "AC" {
            if !ac[p] {
                ac[p] = true;
                ac_cnt += 1;
            }
        } else if !ac[p] {
            wa[p] += 1;
        }
    }

    let wa_cnt = (0..n)
        .filter_map(|i| if ac[i] { Some(wa[i]) } else { None })
        .sum::<usize>();

    println!("{} {}", ac_cnt, wa_cnt);
}
