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

    let mut heap = BinaryHeap::new();
    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: usize,
                }

                queue.push_back(x);
            }
            2 => {
                if let Some(Reverse(x)) = heap.pop() {
                    println!("{}", x);
                } else if let Some(x) = queue.pop_front() {
                    println!("{}", x);
                }
            }
            3 => {
                while let Some(x) = queue.pop_front() {
                    heap.push(Reverse(x));
                }
            }
            _ => unreachable!(),
        }
    }
}
