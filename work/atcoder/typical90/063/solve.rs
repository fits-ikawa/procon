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
        h: usize, w: usize,
        p: [[usize; w]; h],
    }

    use mylib::MultiBTreeSet;

    let mut ans = 0;

    for c in (0..h).powerset() {
        if c.is_empty() {
            continue;
        }

        let mut set = MultiBTreeSet::new();

        for j in 0..w {
            if (0..c.len()).map(|i| p[c[i]][j]).all_equal() {
                set.insert(p[c[0]][j]);
            }
        }

        if !set.is_empty() {
            ans = ans.max(c.len() * set.as_btree_map().values().max().unwrap());
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
        /// The new count of the element in the set after insertion.
        pub fn insert(&mut self, item: T) -> usize {
            let entry = self.map.entry(item).or_insert(0);
            *entry += 1;
            self.size += 1;
            *entry
        }

        /// Removes one occurrence of the specified element.
        ///
        /// # Returns
        /// The new count of the element in the set after removal.
        /// Returns `0` if the element is completely removed from the set.
        pub fn remove(&mut self, item: &T) -> usize {
            if let Some(count) = self.map.get_mut(item) {
                if *count > 1 {
                    *count -= 1;
                    self.size -= 1;
                    *count
                } else {
                    self.map.remove(item);
                    self.size -= 1;
                    0
                }
            } else {
                0
            }
        }

        /// Inserts `n` occurrences of the specified element into the set.
        ///
        /// # Returns
        /// The new count of the element in the set after insertion.
        pub fn insert_n(&mut self, item: T, n: usize) -> usize {
            if n == 0 {
                return self.count(&item);
            }

            let entry = self.map.entry(item).or_insert(0);
            *entry += n;
            self.size += n;
            *entry
        }

        /// Removes up to `n` occurrences of the specified element.
        ///
        /// # Returns
        /// The new count of the element in the set after removal.
        /// Returns `0` if the element is completely removed from the set.
        pub fn remove_n(&mut self, item: &T, n: usize) -> usize {
            if n == 0 {
                return self.count(item);
            }

            if let Some(count) = self.map.get_mut(item) {
                if *count > n {
                    *count -= n;
                    self.size -= n;
                    *count
                } else {
                    self.size -= *count;
                    self.map.remove(item);
                    0
                }
            } else {
                0
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

        /// Returns a reference to the internal BTreeMap.
        ///
        /// # Safety
        /// This exposes the internal structure of the multiset. Use with caution.
        pub fn as_btree_map(&self) -> &BTreeMap<T, usize> {
            &self.map
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
