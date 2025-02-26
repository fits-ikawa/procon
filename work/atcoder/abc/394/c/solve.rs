#![allow(clippy::comparison_chain)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }

    let mut ans = vec![];

    while let Some(last) = s.pop() {
        if !s.is_empty() && s.last().copied().unwrap() == 'W' && last == 'A' {
            s.pop();
            s.push('A');
            s.push('C');
        } else {
            ans.push(last);
        }
    }

    println!("{}", ans.iter().rev().collect::<String>());
}
