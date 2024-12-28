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
        xyc: [(usize, usize, char); m],
    }

    let mut row = btreemap! {};
    let mut col = btreemap! {};

    for (x, y, c) in xyc {
        let rowset = row.entry(x).or_insert(btreeset! {});
        let colset = col.entry(y).or_insert(btreeset! {});

        rowset.insert((y, c));
        colset.insert((x, c));
    }

    if calc(&row, n) && calc(&col, n) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn calc(lines: &BTreeMap<usize, BTreeSet<(usize, char)>>, n: usize) -> bool {
    let mut border = n + 1;

    for line in lines.values() {
        for &(pos, color) in line {
            match color {
                'B' => {
                    if pos >= border {
                        return false;
                    }
                }
                'W' => {
                    border = border.min(pos);
                }
                _ => unreachable!(),
            }
        }
    }

    true
}
