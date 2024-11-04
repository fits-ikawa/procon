#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

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
    }

    let s753 = vec!['7', '5', '3'];
    let mut count = 0;

    for i in 3..=9 {
        for chars in vec![&s753; i].into_iter().multi_cartesian_product() {
            let chars: Vec<char> = chars.into_iter().copied().collect();
            let n_chars = chars.iter().unique().count();
            let n753 = chars.iter().join("").parse::<usize>().unwrap();
            if n_chars == 3 && n753 <= n {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
