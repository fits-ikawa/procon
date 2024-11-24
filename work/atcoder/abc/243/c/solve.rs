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
        n: usize,
        xy: [(usize, usize); n],
        s: Chars,
    }

    let mut lines: HashMap<_, Vec<_>> = hashmap! {};

    for (i, (x, y)) in xy.into_iter().enumerate() {
        lines
            .entry(y)
            .and_modify(|e| e.push((x, i)))
            .or_insert(vec![(x, i)]);
    }

    for (_, line) in lines.iter_mut() {
        line.sort();
    }

    for (_, line) in lines.iter_mut() {
        while !line.is_empty() && s[line.last().unwrap().1] == 'R' {
            line.pop();
        }

        line.reverse();

        while !line.is_empty() && s[line.last().unwrap().1] == 'L' {
            line.pop();
        }

        if line.len() >= 2 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
