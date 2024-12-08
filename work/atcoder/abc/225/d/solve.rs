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

    let mut next = vec![None; n + 1];
    let mut prev = vec![None; n + 1];

    for _ in 0..q {
        input! {
            c: usize,
        }

        match c {
            1 => {
                input! {
                    x: usize, y: usize,
                }

                next[x] = Some(y);
                prev[y] = Some(x);
            }

            2 => {
                input! {
                    x: usize, y: usize,
                }

                next[x] = None;
                prev[y] = None;
            }

            3 => {
                input! {
                    x: usize,
                }

                let mut i = x;
                while prev[i].is_some() {
                    i = prev[i].unwrap();
                }

                let mut ans = vec![];
                loop {
                    ans.push(i);

                    if next[i].is_some() {
                        i = next[i].unwrap()
                    } else {
                        break;
                    }
                }
                print!("{} ", ans.len());
                println!("{}", ans.iter().join(" "));
            }
            _ => unreachable!(),
        }
    }
}
