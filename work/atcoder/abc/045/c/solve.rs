#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

fn main() {
    input! {
        s: Chars,
    }

    let s: Vec<_> = s.into_iter().map(|x| (x as u8 - b'0') as usize).collect();
    let n = s.len() - 1;

    let mut sum = 0;

    for i in 0..1 << n {
        let mut t = s[0];
        for j in 0..n {
            if i & (1 << j) != 0 {
                sum += t;
                t = s[j + 1];
            } else {
                t = t * 10 + s[j + 1];
            }
        }
        sum += t;
    }

    println!("{}", sum);
}
