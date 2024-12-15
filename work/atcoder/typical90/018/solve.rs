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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64, x: f64, y: f64,
        q: usize,
        e: [f64; q],
    }

    for ei in e {
        let (ex, ey, ez) = g(ei, t, l);
        println!(
            "{}",
            ez.atan2(dist3d((ex, ey, 0.0), (x, y, 0.0))).to_degrees()
        );
    }
}

fn g(e: f64, t: f64, l: f64) -> (f64, f64, f64) {
    use std::f64::consts::PI;
    let rad = (e / t) * (PI * 2.0) - PI / 2.0;
    (0.0, -(rad.cos() * l / 2.0), rad.sin() * l / 2.0 + l / 2.0)
}

fn dist3d(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2) + (b.2 - a.2).powi(2)).sqrt()
}
