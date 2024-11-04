#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

fn main() {
    input! {
        cards: [String; 3],
    }

    let mut cards: Vec<Vec<_>> = cards
        .into_iter()
        .map(|x| x.chars().map(|y| y as u8 - b'a').rev().collect())
        .collect();

    let mut turn = 0;

    while let Some(c) = cards[turn].pop() {
        turn = c as usize;
    }

    println!("{}", ['A', 'B', 'C'][turn]);
}
