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
        n: u128,
    }

    for (a, b) in divisors(n) {
        // x^3 - y^3 = (x - y)(x^2 + xy + y^2) = N
        // a = x - y ...(1)
        // b = x^2 + xy + y^2 ...(2)
        // とおく
        //
        // a^2 = x^2 - 2xy + y^2 <= b より
        // a^3 <= ab = N よって a としては
        // n^{1/3} 以下の n の約数を探索すれば十分
        //
        // (1) より y = x - a として (2) から y を消して
        // b = a^3 + 3x^2 - 3ax
        //
        // b <= a^3 + 3x^2 - 3ax となるような最小の x を二分探索し、
        // x^3 - y^3 = N を満たすか確認する
        let mut left = 0;
        let mut right = 2000000000;

        while right - left > 1 {
            let mid = (left + right) / 2;

            if a * a + 3 * mid * mid - 3 * a * mid >= b {
                right = mid;
            } else {
                left = mid;
            }
        }

        let x = right;
        let y = x - a;

        if x > 0 && y > 0 && (x - y) * (x * x + x * y + y * y) == n {
            println!("{} {}", x, y);
            return;
        }
    }

    println!("-1");
}

fn divisors(n: u128) -> Vec<(u128, u128)> {
    let mut pairs = vec![];

    for i in 1..=(n as f64).cbrt().floor() as u128 {
        if n % i == 0 {
            pairs.push((i, n / i));
        }
    }

    pairs
}
