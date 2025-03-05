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
        mut x: isize, mut y: isize, mut z: isize,
    }

    if y < 0 {
        x = -x;
        y = -y;
        z = -z;
    }

    if x < y {
        println!("{}", x.abs());
    } else if z > y {
        println!("-1");
    } else {
        println!("{}", z.abs() + (x - z).abs());
    }
}
