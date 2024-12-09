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
        s: Chars,
    }

    let mut todo = VecDeque::new();
    let mut seen = hashmap! {}; // コストを記録
    todo.push_back(s.clone());
    seen.insert(s, 0);

    let atcoder = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];

    while let Some(mut t) = todo.pop_front() {
        if t == atcoder {
            break;
        }
        let cost = seen[&t];
        for i in 0..6 {
            t.swap(i, i + 1);
            if !seen.contains_key(&t) {
                seen.insert(t.clone(), cost + 1);
                todo.push_back(t.clone());
            }
            t.swap(i, i + 1);
        }
    }

    println!("{}", seen[&atcoder]);
}
