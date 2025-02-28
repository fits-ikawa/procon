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
        q: usize,
    }

    let mut bag = BinaryHeap::new();
    let mut offset = 0;

    for _ in 0..q {
        input! {
            p: usize,
        }

        match p {
            1 => {
                input! {
                    x: isize,
                }

                bag.push(Reverse(x - offset));
            }
            2 => {
                input! {
                    x: isize,
                }

                offset += x;
            }
            3 => {
                let Reverse(x) = bag.pop().unwrap();
                println!("{}", x + offset);
            }
            _ => unreachable!(),
        }
    }
}
