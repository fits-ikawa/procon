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
        n: usize, mut r: isize, mut c: isize,
        s: Chars,
    }

    let (mut ox, mut oy) = (0_isize, 0_isize);
    let mut smoke = hashset! {};
    let mut ans = vec![];

    smoke.insert((0, 0));

    for si in s {
        match si {
            'N' => {
                ox += 1;
                r += 1
            }
            'W' => {
                oy += 1;
                c += 1
            }
            'S' => {
                ox -= 1;
                r -= 1
            }
            'E' => {
                oy -= 1;
                c -= 1
            }
            _ => unreachable!(),
        }
        smoke.insert((ox, oy));
        ans.push(if smoke.contains(&(r, c)) { 1 } else { 0 });
    }

    println!("{}", ans.iter().join(""));
}
