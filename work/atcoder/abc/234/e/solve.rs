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
        x: usize,
    }

    let mut n = 1;

    while x / 10_usize.pow(n) > 0 {
        n += 1;
    }

    let n = n as usize;
    let mut set = btreeset! {};

    let mut add_num = |digits: &[usize]| {
        let mut base = 1;
        let mut num = 0;

        for &d in digits.iter().rev() {
            num += d * base;
            base *= 10;
        }

        if num >= x {
            set.insert(num);
        }
    };

    // 公差が 0
    for j in 1..=9 {
        add_num(&vec![j; n]);
    }

    // 公差が 0 でない
    for j in 0..=9 {
        'next: for k in 1..=9 {
            let mut digits = vec![];
            for l in (j..).step_by(k).take(n) {
                if l >= 10 {
                    continue 'next;
                }
                digits.push(l);
            }
            if digits[0] != 0 {
                add_num(&digits); // 昇順
            }
            digits.reverse();
            add_num(&digits); // 降順
        }
    }

    println!("{}", set.pop_first().unwrap());
}
