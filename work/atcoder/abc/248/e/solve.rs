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
        n: usize, k: usize,
        xy: [(isize, isize); n],
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut map = btreemap! {};

    for i in 0..n {
        let mut group = hashmap! {};

        for j in 0..n {
            if i == j {
                continue;
            }

            let dx = xy[j].0 - xy[i].0;
            let dy = xy[j].1 - xy[i].1;
            let g = dx.gcd(&dy);

            let slope = (dx / g, dy / g);
            let slope = if slope.0 <= 0 {
                (
                    -slope.0,
                    if slope.0 == 0 {
                        slope.1.abs()
                    } else {
                        -slope.1
                    },
                )
            } else {
                slope
            };

            let value = group.entry(slope).or_insert(1);
            *value += 1;
        }

        for &v in group.values() {
            let value = map.entry(v).or_insert(0);
            *value += 1;
        }
    }

    let mut ans = 0;

    for (&k, &v) in map.range(k..) {
        ans += v / k;
    }

    println!("{}", ans);
}
