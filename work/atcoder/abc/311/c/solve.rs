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
        a: [Usize1; n],
    }

    let mut seen = vec![false; n];

    for start in 0..n {
        if !seen[start] {
            let mut todo = VecDeque::new();
            let mut path = vec![];
            todo.push_back(start);
            seen[start] = true;
            path.push(start);

            while let Some(from) = todo.pop_front() {
                let to = a[from];
                if seen[to] {
                    if let Some(pos) = path.iter().position(|&x| x == to) {
                        // 閉路を検出
                        println!("{}", path[pos..].len());
                        println!("{}", path[pos..].iter().map(|x| x + 1).join(" "));
                        return;
                    }
                } else {
                    todo.push_back(to);
                    seen[to] = true;
                    path.push(to);
                }
            }
        }
    }
}
