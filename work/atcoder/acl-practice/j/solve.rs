#![allow(unused_imports)]
use ac_library::{Max, Segtree};
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
#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        a: [isize; n],
    }

    let mut seg_a = Segtree::<Max<_>>::from(a);

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: Usize1, v: isize,
                }
                seg_a.set(x, v);
            }
            2 => {
                input! {
                    l: Usize1, r: Usize1,
                }
                println!("{}", seg_a.prod(l..=r));
            }
            3 => {
                input! {
                    x: Usize1, v: isize,
                }

                let ans = seg_a.max_right(x, |&ai| ai < v) + 1;

                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
