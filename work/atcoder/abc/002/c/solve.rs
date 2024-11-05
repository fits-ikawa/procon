#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

fn main() {
    input! {
        x: (i32, i32),
        y: (i32, i32),
        z: (i32, i32),
    }

    let y = (y.0 - x.0, y.1 - x.1);
    let z = (z.0 - x.0, z.1 - x.1);

    println!("{}", (y.0 * z.1 - y.1 * z.0).abs() as f64 / 2.0);
}
