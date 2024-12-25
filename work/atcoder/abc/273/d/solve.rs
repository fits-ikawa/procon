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
        h: usize, w: usize, rs: Usize1, cs: Usize1,
        n: usize,
        rc: [(Usize1, Usize1); n],
        q: usize,
        dl: [(char, usize); q],
    }

    let mut row = btreemap! {};
    let mut col = btreemap! {};

    for (r, c) in rc {
        let rowset = row.entry(r).or_insert(btreeset! {});
        let colset = col.entry(c).or_insert(btreeset! {});

        (*rowset).insert(c);
        (*colset).insert(r);
    }

    let mut x = rs;
    let mut y = cs;

    for (d, l) in dl {
        match d {
            'L' => {
                if let Some(set) = row.get(&x) {
                    if let Some(&pos) = set.range(..y).next_back() {
                        if pos >= y.saturating_sub(l) {
                            y = pos + 1;
                            println!("{} {}", x + 1, y + 1);
                            continue;
                        }
                    }
                }
                y = y.saturating_sub(l);
                println!("{} {}", x + 1, y + 1);
            }
            'R' => {
                if let Some(set) = row.get(&x) {
                    if let Some(&pos) = set.range(y + 1..).next() {
                        if pos <= y + l {
                            y = pos - 1;
                            println!("{} {}", x + 1, y + 1);
                            continue;
                        }
                    }
                }
                y = (y + l).min(w - 1);
                println!("{} {}", x + 1, y + 1);
            }
            'U' => {
                if let Some(set) = col.get(&y) {
                    if let Some(&pos) = set.range(..x).next_back() {
                        if pos >= x.saturating_sub(l) {
                            x = pos + 1;
                            println!("{} {}", x + 1, y + 1);
                            continue;
                        }
                    }
                }
                x = x.saturating_sub(l);
                println!("{} {}", x + 1, y + 1);
            }
            'D' => {
                if let Some(set) = col.get(&y) {
                    if let Some(&pos) = set.range(x + 1..).next() {
                        if pos <= x + l {
                            x = pos - 1;
                            println!("{} {}", x + 1, y + 1);
                            continue;
                        }
                    }
                }
                x = (x + l).min(h - 1);
                println!("{} {}", x + 1, y + 1);
            }
            _ => unreachable!(),
        }
    }
}
