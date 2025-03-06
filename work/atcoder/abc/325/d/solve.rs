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
        n: usize,
        td: [(usize, usize); n],
    }

    let mut event = btreeset! {};
    let mut machine = btreeset! {};

    for i in 0..n {
        event.insert((td[i].0, 0, i));
    }

    let mut ans = 0;

    while let Some((t, op, i)) = event.pop_first() {
        match op {
            0 => {
                // 商品が印字機に入る
                let deadline = t + td[i].1;
                machine.insert((deadline, i));
                event.insert((t, 1, 0));
                event.insert((deadline, 2, i));
            }
            1 => {
                // 印字する
                if !machine.is_empty() {
                    machine.pop_first();
                    ans += 1;
                    event.insert((t + 1, 1, 0));
                }
            }
            2 => {
                // 商品が印字機から出る
                machine.remove(&(t, i));
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
