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
        tx: [(usize, Usize1); n],
    }

    let mut pot_hist = vec![vec![]; n];
    let mut pot_take = vec![None; n];
    let mut imos = vec![0; n];

    for (i, &(t, x)) in tx.iter().enumerate() {
        match t {
            1 => {
                pot_hist[x].push(i);
                pot_take[i] = Some(0);
            }
            2 => {
                if let Some(j) = pot_hist[x].pop() {
                    pot_take[j] = Some(1);
                    imos[j] += 1;
                    imos[i] -= 1;
                } else {
                    println!("-1");
                    return;
                }
            }
            _ => unreachable!(),
        }
    }

    let pot_n = imos.iter().cumsum::<isize>().max().unwrap();

    println!("{}", pot_n);
    println!("{}", pot_take.iter().flatten().join(" "));
}
