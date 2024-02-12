/// Module `ringbuffer.rs` implements a `RingBuffer` object for managing
/// wrapping fixed-size buffers.
use std::cmp::min;

/// RingBuffer is a wrapper around std::Vec, which holds a predefined amount
/// of items, and for which inserts on a full buffer will push items off the
/// end of the list.
pub struct RingBuffer<T>
where
    T: Clone + Default,
{
    size: usize,
    count: usize,
    head: usize,
    data: Vec<T>,
}

impl<T> RingBuffer<T>
where
    T: Clone + Default,
{
    /// Creates a new [`RingBuffer`] object of size `size`.
    pub fn new(size: usize) -> Self {
        let mut data = Vec::<T>::new();
        data.reserve(size);

        RingBuffer {
            head: 0,
            count: 0,
            size,
            data,
        }
    }

    /// Add the new given item `x` to the front of the ring buffer.
    pub fn add(&mut self, x: T) {
        if self.data.len() < self.size {
            self.data.push(x);
        } else {
            self.data[self.head] = x;
        }

        self.head = (self.head + 1) % self.size;
        self.count = min(self.count + 1, self.size);
    }

    pub fn last(&self) -> Option<&T> {
        match self.count {
            0 => None,
            _ => Some(self.peek(self.head)),
        }
    }

    /// Return ring buffer item at position `idx`.
    ///
    /// Indeces greater than the size of the ring buffer will simply wrap around.
    pub fn peek(&self, idx: usize) -> &T {
        &self.data[idx % self.size]
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

        assert!(rb.peek(0) == &1f32);
        assert!(rb.peek(1) == &2f32);
        assert!(rb.peek(2) == &3f32);
    }

    #[test]
    fn test_use() {
        let mut rb = RingBuffer::<f32>::new(3);

        rb.add(1 as f32);
        rb.add(2 as f32);
        rb.add(3 as f32);

        assert!(rb.peek(0) == &1f32);
        assert!(rb.peek(1) == &2f32);
        assert!(rb.peek(2) == &3f32);

        rb.add(4 as f32);
        rb.add(5 as f32);
        rb.add(6 as f32);

        assert!(rb.peek(0) == &4f32);
        assert!(rb.peek(1) == &5f32);
        assert!(rb.peek(2) == &6f32);
    }
}
