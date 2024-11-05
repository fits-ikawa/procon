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
        data: [(usize, usize); n],
    }

    let mut imos = [0; 1000000 + 2];

    for (s, e) in data {
        imos[s] += 1;
        imos[e + 1] -= 1;
    }

    let sum: Vec<_> = imos
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    println!("{}", sum.iter().max().unwrap());
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        data: [(usize, usize); n],
    }

    let (mut starts, mut ends): (Vec<usize>, Vec<usize>) = data.iter().cloned().unzip();

    starts.sort_by(|a, b| b.cmp(a));
    ends.sort_by(|a, b| b.cmp(a));

    let mut cur_n = 0;
    let mut max_n = 0;

    for i in 0..=1000000 {
        while let Some(&s) = starts.last() {
            if s == i {
                cur_n += 1;
                max_n = max_n.max(cur_n);
                starts.pop();
            } else {
                break;
            }
        }
        while let Some(&e) = ends.last() {
            if e == i {
                cur_n -= 1;
                ends.pop();
            } else {
                break;
            }
        }
    }

    println!("{}", max_n);
}
