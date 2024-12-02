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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; m],
        s: [Chars; n],
    }

    let points = (0..n)
        .map(|i| {
            a.iter()
                .enumerate()
                .filter_map(|(j, &aj)| if s[i][j] == 'o' { Some(aj) } else { None })
                .sum::<usize>()
                + i
                + 1
        })
        .collect_vec();

    let top = points.iter().max().copied().unwrap();

    for i in 0..n {
        if points[i] >= top {
            println!("0");
            continue;
        }

        let todo = a
            .iter()
            .enumerate()
            .filter_map(|(j, &aj)| if s[i][j] == 'x' { Some(aj) } else { None })
            .sorted()
            .rev()
            .cumsum::<usize>()
            .collect_vec();

        let diff = top - points[i];
        println!("{}", todo.lower_bound(&diff) + 1);
    }
}
