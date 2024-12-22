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
        a: usize, n: usize,
    }

    let mut todo = VecDeque::new();
    let mut seen = btreemap! {};
    todo.push_back(1);
    seen.insert(1, 0);

    while let Some(from) = todo.pop_front() {
        let multiplied = from * a;
        if multiplied < 1000000 && !seen.contains_key(&(multiplied)) {
            seen.insert(multiplied, seen[&from] + 1);
            todo.push_back(multiplied);
        }

        let rotated = rotate(from);
        if from != rotated && !seen.contains_key(&rotated) {
            seen.insert(rotated, seen[&from] + 1);
            todo.push_back(rotated);
        }
    }

    if let Some(cost) = seen.get(&n) {
        println!("{}", cost);
    } else {
        println!("-1");
    }
}

fn rotate(n: usize) -> usize {
    if n < 10 || n % 10 == 0 {
        return n;
    }

    if n < 100 {
        (n % 10) * 10 + n / 10
    } else if n < 1000 {
        (n % 10) * 100 + n / 10
    } else if n < 10000 {
        (n % 10) * 1000 + n / 10
    } else if n < 100000 {
        (n % 10) * 10000 + n / 10
    } else if n < 1000000 {
        (n % 10) * 100000 + n / 10
    } else {
        unreachable!()
    }
}
