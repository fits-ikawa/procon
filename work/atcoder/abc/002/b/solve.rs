#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

fn main() {
    input! {
        w: String,
    }

    let vowels = hashset! {'a', 'i', 'u', 'e', 'o'};

    println!(
        "{}",
        w.chars()
            .filter(|x| !vowels.contains(x))
            .collect::<String>()
    );
}
