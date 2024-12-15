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
    // 解説 AC
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut ls = vec![];
    let mut rs = vec![];

    for (l, r) in lr {
        ls.push(l);
        rs.push(r);
    }

    ls.sort();
    rs.sort();

    let mut ans = n * (n - 1) / 2;
    let mut j = 0;

    for i in 0..n {
        while rs[j] < ls[i] {
            j += 1;
        }
        ans -= j;
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut l2r = btreemap! {};

    for (l, r) in lr {
        (*l2r.entry(l).or_insert(vec![])).push(r);
    }

    let mut rlive = mylib::MultiBTreeSet::new();
    let mut ans = 0;

    for &left in l2r.keys() {
        // 見終わった区間を捨てる
        while !rlive.is_empty() && rlive.first().copied().unwrap() < left {
            rlive.pop_first();
        }

        // 残っている区間と今回追加する区間の組み合わせ
        ans += l2r[&left].len() * rlive.len();
        // 今回追加する区間同士の組み合わせ
        ans += l2r[&left].len() * (l2r[&left].len() - 1) / 2;

        // 今回の区間を追加する
        for &r in &l2r[&left] {
            rlive.insert(r);
        }
    }

    println!("{}", ans);
}

pub mod mylib {
    use std::collections::{btree_map, BTreeMap};
    use std::iter::{FlatMap, Repeat, Take};

    /// A multiset implemented using a `BTreeMap`.
    ///
    /// This data structure allows for duplicate elements while maintaining
    /// elements in sorted order. It internally tracks the count of each element.
    #[derive(Debug, PartialEq, Clone)]
    pub struct MultiBTreeSet<T> {
        map: BTreeMap<T, usize>,
        size: usize,
    }

    impl<T: Ord> Default for MultiBTreeSet<T> {
        /// Creates an empty `MultiBTreeSet`.
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Ord> std::iter::FromIterator<T> for MultiBTreeSet<T> {
        /// Creates a `MultiBTreeSet` from an iterator.
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut set = Self::new();
            for item in iter {
                set.insert(item);
            }
            set
        }
    }

    impl<T: Ord> MultiBTreeSet<T> {
        /// Creates a new empty `MultiBTreeSet`.
        pub fn new() -> Self {
            Self {
                map: BTreeMap::new(),
                size: 0,
            }
        }

        /// Removes all elements from the set.
        pub fn clear(&mut self) {
            self.map.clear();
            self.size = 0;
        }

        /// Inserts an element into the set.
        ///
        /// # Returns
        /// `true` if the element is newly added to the set (was not present before).
        pub fn insert(&mut self, item: T) -> bool {
            let entry = self.map.entry(item).or_insert(0);
            let was_new = *entry == 0;
            *entry += 1;
            self.size += 1;
            was_new
        }

        /// Removes one occurrence of the specified element.
        ///
        /// # Returns
        /// The new count of the element, or `None` if the element was not present.
        pub fn remove(&mut self, item: &T) -> Option<usize> {
            if let Some(count) = self.map.get_mut(item) {
                if *count > 1 {
                    *count -= 1;
                    self.size -= 1;
                    Some(*count)
                } else {
                    self.map.remove(item);
                    self.size -= 1;
                    Some(0)
                }
            } else {
                None
            }
        }

        /// Checks if the set contains the specified element.
        pub fn contains(&self, item: &T) -> bool {
            self.map.contains_key(item)
        }

        /// Returns the count of the specified element in the set.
        pub fn count(&self, item: &T) -> usize {
            *self.map.get(item).unwrap_or(&0)
        }

        /// Checks if the set is empty.
        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        /// Returns the total count of all elements in the set, including duplicates.
        pub fn len(&self) -> usize {
            self.size
        }

        /// Returns the count of unique elements in the set.
        pub fn len_unique(&self) -> usize {
            self.map.len()
        }

        /// Returns a reference to the smallest element in the set.
        pub fn first(&self) -> Option<&T> {
            self.map.iter().next().map(|(k, _)| k)
        }

        /// Returns a reference to the largest element in the set.
        pub fn last(&self) -> Option<&T> {
            self.map.iter().next_back().map(|(k, _)| k)
        }

        /// Removes and returns the smallest element in the set.
        pub fn pop_first(&mut self) -> Option<T>
        where
            T: Clone,
        {
            if let Some(mut entry) = self.map.first_entry() {
                let item = entry.key().clone();
                if *entry.get() > 1 {
                    *entry.get_mut() -= 1;
                } else {
                    entry.remove();
                }
                self.size -= 1;
                Some(item)
            } else {
                None
            }
        }

        /// Removes and returns the largest element in the set.
        pub fn pop_last(&mut self) -> Option<T>
        where
            T: Clone,
        {
            if let Some(mut entry) = self.map.last_entry() {
                let item = entry.key().clone();
                if *entry.get() > 1 {
                    *entry.get_mut() -= 1;
                } else {
                    entry.remove();
                }
                self.size -= 1;
                Some(item)
            } else {
                None
            }
        }

        /// Returns an iterator over unique elements in the specified range.
        pub fn range_unique<R>(&self, range: R) -> impl Iterator<Item = (&T, &usize)>
        where
            R: core::ops::RangeBounds<T>,
        {
            self.map.range(range)
        }

        /// Returns an iterator over all elements in the specified range, including duplicates.
        pub fn range<R>(&self, range: R) -> impl Iterator<Item = &T>
        where
            R: core::ops::RangeBounds<T>,
        {
            self.map
                .range(range)
                .flat_map(|(item, &count)| std::iter::repeat(item).take(count))
        }

        /// Returns an iterator over unique elements in the set.
        pub fn iter_unique(&self) -> impl Iterator<Item = (&T, &usize)> {
            self.map.iter()
        }

        /// Consumes the set and returns an iterator over unique elements.
        pub fn into_iter_unique(self) -> impl Iterator<Item = (T, usize)> {
            self.map.into_iter()
        }

        /// Returns an iterator over all elements in the set, including duplicates.
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.map
                .iter()
                .flat_map(|(item, &count)| std::iter::repeat(item).take(count))
        }
    }

    impl<'a, T: Ord> IntoIterator for &'a MultiBTreeSet<T> {
        type Item = &'a T;
        type IntoIter = FlatMap<
            btree_map::Iter<'a, T, usize>,
            Take<Repeat<&'a T>>,
            fn((&'a T, &'a usize)) -> Take<Repeat<&'a T>>,
        >;

        /// Converts the set into an iterator over references to all elements, including duplicates.
        fn into_iter(self) -> Self::IntoIter {
            self.map
                .iter()
                .flat_map(|(item, &count)| std::iter::repeat(item).take(count))
        }
    }

    impl<T: Ord + Clone> IntoIterator for MultiBTreeSet<T> {
        type Item = T;
        type IntoIter = FlatMap<
            btree_map::IntoIter<T, usize>,
            Take<Repeat<T>>,
            fn((T, usize)) -> Take<Repeat<T>>,
        >;

        /// Consumes the set and returns an iterator over all elements, including duplicates.
        fn into_iter(self) -> Self::IntoIter {
            self.map
                .into_iter()
                .flat_map(|(item, count)| std::iter::repeat(item).take(count))
        }
    }
}
