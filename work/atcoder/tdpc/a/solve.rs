#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

fn main() {
    input! {
        n: usize,
        vec: [usize; n],
    }

    let m: usize = vec.iter().sum();

    let mut dp = vec![false; m + 1];
    dp[0] = true;

    for s in vec {
        for i in (0..=m - s).rev() {
            dp[i + s] = dp[i] || dp[i + s];
        }
    }

    println!("{}", dp.iter().filter(|&&x| x).count());
}
