use std::cmp::min;

use num_traits::Zero;

/// RingBuffer is a wrapper around std::Vec, which holds a predefined amount
/// of items, and for which inserts on a full buffer will push items off the
/// end of the list.
pub struct RingBuffer<T>
where
    T: Copy + Default,
{
    size: usize,
    count: usize,
    head: usize,
    data: Vec<T>,
}

impl<T> RingBuffer<T>
where
    T: Copy + Default,
{
    /// Creates a new [`RingBuffer`] object of size `size`.
    pub fn new(size: usize) -> Self {
        let mut data = Vec::<T>::new();
        data.reserve(size);

        RingBuffer {
            head: 0,
            count: 0,
            size,
            data: vec![T::default(); size],
        }
    }

    /// Add the new given item `x` to the front of the ring buffer.
    pub fn add(&mut self, x: T) {
        self.data[self.head] = x;
        self.head = (self.head + 1) % self.size;
        self.count = min(self.count + 1, self.size);
    }

    /// Return ring buffer item at position `idx`.
    ///
    /// Indeces greater than the size of the ring buffer will simply wrap around.
    pub fn peek(&self, idx: usize) -> T {
        self.data[idx % self.size]
    }

    /// Get the length of the ring buffer
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the count of current items in the ring buffer.
    ///
    /// Note that this will never be less than 0, nor more than the size of
    /// the buffer.
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
pub mod tests {
    use super::RingBuffer;

    #[test]
    fn test_create() {
        let rb = RingBuffer::<f32>::new(10);

        assert!(rb.size() == 10);
        assert!(rb.count() == 0);
    }

    #[test]
    fn test_add() {
        let mut rb = RingBuffer::<f32>::new(5);
        rb.add(1.0);
        rb.add(2.0);
        rb.add(3.0);

        assert!(rb.count() == 3);
        assert!(rb.size() == 5);

        rb.add(0.0);
        rb.add(0.0);
        rb.add(0.0);
        rb.add(0.0);

        assert!(rb.count() == 5);
        assert!(rb.size() == 5);
        assert!(rb.data.len() == 5);
    }

    #[test]
    fn test_peek() {
        let mut rb = RingBuffer::<f32>::new(3);
        rb.add(1 as f32);
        rb.add(2 as f32);
        rb.add(3 as f32);

        assert!(rb.peek(0) == 1 as f32);
        assert!(rb.peek(1) == 2 as f32);
        assert!(rb.peek(2) == 3 as f32);
    }
}
