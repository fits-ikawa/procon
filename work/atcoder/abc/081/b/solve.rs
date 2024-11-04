#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut count = 0;

    while a.iter().all(|x| x % 2 == 0) {
        a = a.into_iter().map(|x| x >> 1).collect();
        count += 1;
    }

    println!("{}", count);
}
