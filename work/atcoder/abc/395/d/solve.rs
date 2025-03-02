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
        n: usize, q: usize,
    }

    let mut p2i = (0..n).collect_vec();
    let mut i2n = (0..n).collect_vec();
    let mut n2i = (0..n).collect_vec();

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    a: Usize1, b: Usize1,
                }

                let b_i = n2i[b];
                p2i[a] = b_i;
            }
            2 => {
                input! {
                    a: Usize1, b: Usize1,
                }

                let a_i = n2i[a];
                let b_i = n2i[b];
                n2i[a] = b_i;
                n2i[b] = a_i;

                let a_n = i2n[a_i];
                let b_n = i2n[b_i];
                i2n[a_i] = b_n;
                i2n[b_i] = a_n;
            }
            3 => {
                input! {
                    a: Usize1,
                }

                println!("{}", i2n[p2i[a]] + 1);
            }
            _ => unreachable!(),
        }
    }
}
