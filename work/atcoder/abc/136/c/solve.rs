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
    // 解説 AC
    // 後ろから操作
    input! {
        n: usize,
        mut h: [usize; n],
    }

    for i in (0..n - 1).rev() {
        if h[i] > h[i + 1] {
            if h[i] == h[i + 1] + 1 {
                h[i] -= 1;
            } else {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut up = true;

    for i in 1..n {
        if h[i - 1] > h[i] {
            if h[i - 1] == h[i] + 1 && up {
                up = false;
            } else {
                println!("No");
                return;
            }
        } else if h[i - 1] < h[i] {
            up = true;
        }
    }

    println!("Yes");
}
