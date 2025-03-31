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
        n: usize, q: usize,
    }

    let mut a = (1..=n).collect_vec();
    let mut reverse = false;

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: Usize1, y: usize,
                }

                let i = if reverse { n - 1 - x } else { x };
                a[i] = y;
            }
            2 => {
                reverse = !reverse;
            }
            3 => {
                input! {
                    x: Usize1,
                }

                let i = if reverse { n - 1 - x } else { x };
                println!("{}", a[i]);
            }
            _ => unreachable!(),
        }
    }
}
