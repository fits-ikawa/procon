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
        s: Chars,
    }

    let mut s_enc = vec![0; 10];

    for si in s {
        s_enc[(si as u8 - b'0') as usize] += 1;
    }

    let mut map = hashmap! {};

    for i in 0_usize.. {
        let mut i2 = i * i;

        if i2 >= 10_usize.pow(n as u32) {
            break;
        }

        let mut enc = vec![0; 10];
        let mut cnt = 0;

        while i2 > 0 {
            enc[i2 % 10] += 1;
            cnt += 1;
            i2 /= 10;
        }

        enc[0] += n - cnt;

        let value = map.entry(enc).or_insert(0);
        *value += 1;
    }

    let ans = map.get(&s_enc).unwrap_or(&0);

    println!("{}", ans);
}
