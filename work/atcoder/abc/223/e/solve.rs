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
        x: usize, y: usize, a: usize, b: usize, c: usize,
    }

    for (xx, yy) in [(x, y), (y, x)] {
        for (aa, bb, cc) in [(a, b, c), (b, a, c), (c, a, b)] {
            let check = || {
                let d = (aa + yy - 1) / yy;

                if d < xx {
                    let f = xx - d;
                    let e = (bb + yy - 1) / yy;
                    if e < f && (f - e) * yy >= cc {
                        return true;
                    }

                    let e = (bb + f - 1) / f;
                    if e < yy && f * (yy - e) >= cc {
                        return true;
                    }
                }

                false
            };

            if check() {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
