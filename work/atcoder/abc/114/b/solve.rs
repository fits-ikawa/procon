#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

fn main() {
    input! {
        s: Chars
    }

    let answer = s
        .windows(3)
        .map(|w| w.iter().join("").parse::<usize>().unwrap().abs_diff(753))
        .min()
        .unwrap();

    println!("{}", answer);
}
