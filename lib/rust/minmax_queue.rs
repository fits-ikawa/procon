pub mod mylib {
    use std::collections::VecDeque;

    /// A monotonic queue that efficiently tracks the minimum element in a sliding window.
    ///
    /// # Usage
    /// - Push and pop must be called in FIFO order.
    /// - This queue maintains a non-decreasing order of elements.
    ///
    /// # Complexity
    /// - O(n) total for n elements.
    pub struct MinQueue<T: Ord + Copy> {
        deque: VecDeque<T>,
    }

    impl<T: Ord + Copy> MinQueue<T> {
        /// Creates a new empty `MinQueue`.
        pub fn new() -> Self {
            Self {
                deque: VecDeque::new(),
            }
        }

        /// Pushes a new element into the queue.
        ///
        /// Maintains internal monotonic (non-decreasing) order.
        pub fn push(&mut self, value: T) {
            while let Some(&back) = self.deque.back() {
                if back > value {
                    self.deque.pop_back();
                } else {
                    break;
                }
            }
            self.deque.push_back(value);
        }

        /// Pops the front element from the queue if it matches `value`.
        ///
        /// Used to remove the oldest element from the window.
        pub fn pop(&mut self, value: T) {
            if let Some(&front) = self.deque.front() {
                if front == value {
                    self.deque.pop_front();
                }
            }
        }

        /// Returns the current minimum element in the queue.
        pub fn min(&self) -> Option<T> {
            self.deque.front().copied()
        }
    }

    impl<T: Ord + Copy> Default for MinQueue<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    /// A monotonic queue that efficiently tracks the maximum element in a sliding window.
    ///
    /// # Usage
    /// - Push and pop must be called in FIFO order.
    /// - This queue maintains a non-increasing order of elements.
    ///
    /// # Complexity
    /// - O(n) total for n elements.
    pub struct MaxQueue<T: Ord + Copy> {
        deque: VecDeque<T>,
    }

    impl<T: Ord + Copy> MaxQueue<T> {
        /// Creates a new empty `MaxQueue`.
        pub fn new() -> Self {
            Self {
                deque: VecDeque::new(),
            }
        }

        /// Pushes a new element into the queue.
        ///
        /// This maintains the internal monotonic (non-increasing) order.
        pub fn push(&mut self, value: T) {
            while let Some(&back) = self.deque.back() {
                if back < value {
                    self.deque.pop_back();
                } else {
                    break;
                }
            }
            self.deque.push_back(value);
        }

        /// Pops the front element from the queue if it matches `value`.
        ///
        /// Used to remove the oldest element from the window.
        pub fn pop(&mut self, value: T) {
            if let Some(&front) = self.deque.front() {
                if front == value {
                    self.deque.pop_front();
                }
            }
        }

        /// Returns the current maximum element in the queue.
        pub fn max(&self) -> Option<T> {
            self.deque.front().copied()
        }
    }

    impl<T: Ord + Copy> Default for MaxQueue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
