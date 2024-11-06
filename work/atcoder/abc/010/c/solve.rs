#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        txa: i32, tya: i32,
        txb: i32, tyb: i32,
        t: i32, v: i32,
        n: usize,
        girls: [(i32, i32); n],
    }

    for (gx, gy) in girls {
        let d = (((gx - txa).pow(2) + (gy - tya).pow(2)) as f64).sqrt()
            + (((txb - gx).pow(2) + (tyb - gy).pow(2)) as f64).sqrt();

        if d <= (v * t) as f64 {
            println!("YES");
            return;
        }
    }

    println!("NO");
}
