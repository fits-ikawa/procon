#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        mut x: usize, mut y: usize,
    }

    let mut ans = vec![[x, y]];

    while x > 1 || y > 1 {
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
        ans.push([x, y]);
    }

    ans.pop();
    ans.reverse();

    println!("{}", ans.len());

    if !ans.is_empty() {
        println!(
            "{}",
            ans.iter().map(|line| line.iter().join(" ")).join("\n")
        );
    }
}
