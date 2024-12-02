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
        _n: usize,
        q: usize,
    }

    let mut box2cards = hashmap! {};
    let mut card2boxes = hashmap! {};

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    i: usize, j: usize,
                }

                let v = box2cards.entry(j).or_insert(btreemap! {});
                *v.entry(i).or_insert(0) += 1;

                let v = card2boxes.entry(i).or_insert(btreeset! {});
                v.insert(j);
            }
            2 => {
                input! {
                    i:usize,
                }

                if let Some(v) = box2cards.get(&i) {
                    println!("{}", v.iter().flat_map(|(&k, &v)| vec![k; v]).join(" "));
                } else {
                    println!();
                }
            }
            3 => {
                input! {
                    i: usize,
                }

                if let Some(v) = card2boxes.get(&i) {
                    println!("{}", v.iter().join(" "));
                } else {
                    println!();
                }
            }
            _ => unreachable!(),
        }
    }
}
