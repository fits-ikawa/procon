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
        n: usize,
        ab: [(isize, isize); n],
        cd: [(isize, isize); n],
    }

    if n == 1 {
        println!("Yes");
        return;
    }

    if n == 2 {
        println!(
            "{}",
            if distsq(ab[0], ab[1]) == distsq(cd[0], cd[1]) {
                "Yes"
            } else {
                "No"
            }
        );
        return;
    }

    let (ab1, ab2) = (ab[0], ab[1]);

    for cd_perm in cd.iter().permutations(2) {
        let (&cd1, &cd2) = (cd_perm[0], cd_perm[1]);
        if distsq(ab1, ab2) == distsq(cd1, cd2) {
            // 2 点の対応がついた
            let mut cd_set = cd.iter().copied().collect::<HashSet<_>>();
            cd_set.remove(&cd1);
            cd_set.remove(&cd2);

            for i in 2..n {
                let ab3 = ab[i];
                let mut paired = None;
                for &cd3 in &cd_set {
                    if distsq(ab1, ab3) == distsq(cd1, cd3) && distsq(ab2, ab3) == distsq(cd2, cd3)
                    {
                        // 2 点から 3 点目への距離が一致
                        // 外積で位置関係の一致を調べる
                        let ab_e1 = vec_sub(ab2, ab1);
                        let ab_e2 = vec_sub(ab3, ab1);
                        let cd_e1 = vec_sub(cd2, cd1);
                        let cd_e2 = vec_sub(cd3, cd1);
                        if cross(ab_e1, ab_e2).signum() == cross(cd_e1, cd_e2).signum() {
                            // 3 点の位置関係まで一致したので対応がついた
                            paired = Some(cd3);
                            break;
                        }
                    }
                }
                if let Some(cd3) = paired {
                    cd_set.remove(&cd3);
                } else {
                    break;
                }
            }

            if cd_set.is_empty() {
                // 全て対応がついた
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn distsq(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

fn vec_sub(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 - b.0, a.1 - b.1)
}

fn cross(a: (isize, isize), b: (isize, isize)) -> isize {
    a.0 * b.1 - b.0 * a.1
}
