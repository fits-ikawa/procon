#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

fn main() {
    input! {
        k: i32, s: i32,
    }

    let mut count = 0;

    for (x, y) in iproduct!(0..=k, 0..=k) {
        let remain = s - (x + y);
        if 0 <= remain && remain <= k {
            count += 1;
        }
    }

    println!("{}", count);
}
