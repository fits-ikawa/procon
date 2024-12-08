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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut tube = vec![];
    let mut count = 0;

    for ai in a {
        if let Some(&(k, v)) = tube.last() {
            if k == ai {
                tube.pop();
                match ai.cmp(&(v + 1)) {
                    Less => {
                        tube.push((k, v + 1 - ai));
                        count -= ai - 1;
                    }
                    Equal => {
                        count -= v;
                    }
                    Greater => {
                        tube.push((k, v + 1));
                        count += 1;
                    }
                }
            } else {
                tube.push((ai, 1));
                count += 1;
            }
        } else {
            tube.push((ai, 1));
            count += 1;
        }

        println!("{}", count);
    }
}
