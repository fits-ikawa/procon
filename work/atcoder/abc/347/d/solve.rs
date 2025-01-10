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
        a: usize, b: usize, c: usize,
    }

    let diff = a.abs_diff(b);
    let c1 = c.count_ones() as usize;

    if diff <= c1 && diff % 2 == c1 % 2 {
        let both = (c1 - diff) / 2;
        if a.min(b) - both <= 60 - c1 {
            let mut x = 0;
            let mut y = 0;

            let mut s = a.min(b) - both;
            let mut t = diff;
            let mut u = true;

            for i in 0..60 {
                let d = 1_usize << i;
                if c & d == 0 {
                    if s > 0 {
                        x += d;
                        y += d;
                        s -= 1;
                    }
                } else if t > 0 {
                    if a >= b {
                        x += d;
                    } else {
                        y += d;
                    }
                    t -= 1;
                } else {
                    if u {
                        x += d;
                    } else {
                        y += d;
                    }
                    u = !u;
                }
            }

            println!("{} {}", x, y);
            return;
        }
    }

    println!("-1");
}
