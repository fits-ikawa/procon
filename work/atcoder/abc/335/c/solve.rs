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
        n: usize, q: usize,
    }

    let mut dragon = VecDeque::new();

    for i in 0..n {
        dragon.push_back(((i + 1) as isize, 0_isize));
    }

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    c: char,
                }

                let (dx, dy) = match c {
                    'R' => (1, 0),
                    'L' => (-1, 0),
                    'U' => (0, 1),
                    'D' => (0, -1),
                    _ => unreachable!(),
                };

                let (x, y) = dragon.front().unwrap();

                dragon.push_front((x + dx, y + dy));
                dragon.pop_back();
            }
            2 => {
                input! {
                    p: usize,
                }

                let (x, y) = dragon[p - 1];
                println!("{} {}", x, y);
            }
            _ => unreachable!(),
        }
    }
}
