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
        n: usize, m: usize,
        sc: [(Usize1, usize); m],
    }

    for x in 0..1000 {
        let d = if x == 0 {
            vec![Some(0)]
        } else {
            let mut tmp = vec![];
            let mut y = x;

            while y > 0 {
                tmp.push(Some(y % 10));
                y /= 10;
            }

            tmp.reverse();
            tmp
        };

        if n == d.len() && sc.iter().all(|&(s, c)| d[s] == Some(c)) {
            println!("{}", x);
            return;
        }
    }

    println!("-1");
}
