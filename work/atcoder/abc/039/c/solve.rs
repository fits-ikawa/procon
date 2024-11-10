#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use proconio::{marker::*, *};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        s: String,
    }

    let mut octave = "WBWBWWBWBWBW".chars().collect::<VecDeque<_>>();

    for note in [
        "Do", "Do#", "Re", "Re#", "Mi", "Fa", "Fa#", "So", "So#", "La", "La#", "Si",
    ] {
        if s[..12] == octave.iter().collect::<String>() {
            println!("{}", note);
            return;
        }
        let key = octave.pop_front().unwrap();
        octave.push_back(key);
    }
}
