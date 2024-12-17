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
        b: [usize; n],
        c: [usize; n],
    }

    let a_cnt = a.iter().map(|&x| x % 46).counts();
    let b_cnt = b.iter().map(|&x| x % 46).counts();
    let c_cnt = c.iter().map(|&x| x % 46).counts();

    let mut ans = 0;

    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += a_cnt.get(&i).unwrap_or(&0)
                        * b_cnt.get(&j).unwrap_or(&0)
                        * c_cnt.get(&k).unwrap_or(&0);
                }
            }
        }
    }

    println!("{}", ans);
}
