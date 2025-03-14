#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use ac_library::ModInt998244353;
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
        n: u64, k: u64, m: u64,
    }

    use ac_library::{ModInt, ModInt998244353};

    if ModInt998244353::new(m).val() == 0 {
        println!("0");
        return;
    }

    ModInt::set_modulus(998244352);
    let r = ModInt::new(k).pow(n).val() as u64;

    println!("{}", ModInt998244353::new(m).pow(r));
}
