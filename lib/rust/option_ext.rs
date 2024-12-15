pub mod mylib {
    use std::cmp::Ord;

    pub trait OptionExt<T> {
        /// Returns the minimum of two `Option` values.
        /// If both are `Some`, returns the minimum value. If one is `None`, returns the other.
        fn min_or(self, other: Option<T>) -> Option<T>;

        /// Returns the maximum of two `Option` values.
        /// If both are `Some`, returns the maximum value. If one is `None`, returns the other.
        fn max_or(self, other: Option<T>) -> Option<T>;
    }

    impl<T: Ord> OptionExt<T> for Option<T> {
        fn min_or(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(x), Some(y)) => Some(x.min(y)),
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        }

        fn max_or(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(x), Some(y)) => Some(x.max(y)),
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        }
    }
}
