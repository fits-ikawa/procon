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
        a: [usize; n],
    }

    let mut bit = ac_library::FenwickTree::new(n, 0_usize);

    for i in 0..n {
        bit.add(i, a[i]);
    }

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            0 => {
                input! {
                    p: usize, x: usize,
                }

                bit.add(p, x);
            }
            1 => {
                input! {
                    l: usize, r: usize,
                }

                println!("{}", bit.sum(l..r));
            }
            _ => unreachable!(),
        }
    }
}
