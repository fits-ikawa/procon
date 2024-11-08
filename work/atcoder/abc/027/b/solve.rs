#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use proconio::{marker::*, *};
use std::collections::*;
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
        n: usize,
        a: [usize; n],
    }

    let total: usize = a.iter().sum();

    if total % n != 0 {
        println!("-1");
        return;
    }

    let per_island = total / n;
    let mut ci = 1;
    let mut cn = a[0];
    let mut ans = 0;

    for &ai in &a[1..n] {
        if cn == ci * per_island {
            ci = 1;
            cn = ai;
        } else {
            ans += 1;
            ci += 1;
            cn += ai;
        }
    }

    println!("{}", ans);
}
