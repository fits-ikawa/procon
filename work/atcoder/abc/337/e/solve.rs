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

fn main() {
    input_interactive! {
        n: usize,
    }

    let m = n.next_power_of_two().ilog2() as usize;
    let mut juice = vec![vec![]; m];

    for i in 0..n {
        for j in 0..m {
            if (i >> j) & 1 > 0 {
                juice[j].push(i + 1);
            }
        }
    }

    println!("{}", m);

    // s を最上位ビットから受け取るため、逆順に指定する
    for j in juice.iter().rev() {
        println!("{} {}", j.len(), j.iter().join(" "));
    }

    input_interactive! {
        s: String,
    }

    let x = usize::from_str_radix(&s, 2).unwrap() + 1;

    println!("{}", x);
}
