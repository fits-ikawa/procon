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
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut first = a[0];
    let mut prev = hashmap! {};
    let mut next = hashmap! {};

    if n == 1 {
        prev.insert(a[0], None);
        next.insert(a[0], None);
    } else {
        for i in 0..n {
            if i == 0 {
                prev.insert(a[i], None);
                next.insert(a[i], Some(a[i + 1]));
            } else if i == n - 1 {
                prev.insert(a[i], Some(a[i - 1]));
                next.insert(a[i], None);
            } else {
                prev.insert(a[i], Some(a[i - 1]));
                next.insert(a[i], Some(a[i + 1]));
            }
        }
    }

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: usize, y: usize,
                }

                let next_x = next.get(&x).copied().unwrap();

                next.insert(x, Some(y));
                next.insert(y, next_x);
                prev.insert(y, Some(x));

                if let Some(nx) = next_x {
                    prev.insert(nx, Some(y));
                }
            }
            2 => {
                input! {
                    x: usize,
                }

                let prev_x = prev.get(&x).copied().unwrap();
                let next_x = next.get(&x).copied().unwrap();

                match (prev_x, next_x) {
                    (None, Some(nx)) => {
                        prev.insert(nx, None);
                        first = nx;
                    }
                    (Some(px), Some(nx)) => {
                        next.insert(px, Some(nx));
                        prev.insert(nx, Some(px));
                    }
                    (Some(px), None) => {
                        next.insert(px, None);
                    }
                    (None, None) => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    let mut ans = vec![];
    let mut cur = first;

    loop {
        ans.push(cur);

        if let Some(nx) = next.get(&cur).copied().unwrap() {
            cur = nx;
        } else {
            break;
        }
    }

    println!("{}", ans.iter().join(" "));
}
