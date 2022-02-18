//!  CircularVector implements a cyclic data structure where elements
//!  after the end of the vector are mapped back to beginning.  This
//!  type of data structure is sometimes called a FIFO, a queue, shift register,
//!  a ring buffer, or a delay line
//!
//!  # Applications
//!
//!  # Example
//!  In this implementation, the number of elements, n, is fixed.  
//!  Indices into the underlying data are computed modulo n such that
//!  the element \[n\] refers to element\[0\] etc.

use std::fmt::Debug;

pub struct CircularVector<T: Copy + Debug> {
    tail: usize,  // Index of the most recently added element
    data: Vec<T>, // Vector containing the data.
}

// TODO:

impl<T: Copy + Debug> CircularVector<T> {
    /// Creates a new CircularVector of with length `length` and initialized to `init_value`.
    /// ## Errors
    /// If the requested length is zero, the Result will contain an error
    pub fn new(length: usize, init_value: T) -> Result<Self, String> {
        if length == 0 {
            return Err("Size must be greater than zero".to_string());
        } else {
            Ok(Self {
                tail: length - 1,               // raw index to the last element
                data: vec![init_value; length], //
            })
        }
    }
    /// Returns the element at index, where 0 is the head of the vector
    pub fn get(&self, index: usize) -> T {
        self.data[self.circular_index(self.tail + 1 + index)]
    }
    /// Returns the length
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Pushes an element at the end (tail).  The element at the head is popped off and returned
    pub fn push(&mut self, x: T) -> T {
        let head_index = self.circular_index(self.tail + 1); // Head index always follows the tail inxex
        let head_value = self.data[head_index]; // Save head value to return later
        self.data[head_index] = x; // Overwrite the head with the new tail value
        self.tail = head_index; // Update the tail index to point to the new location
        head_value // Return the old (popped) head value
    }
    /// Used internally to compute the index modulo length into the raw data
    fn circular_index(&self, i: usize) -> usize {
        i % self.len() // Might want to cache the length for speed
    }
    /// Print the contents of the CircularVector starting from the head
    /// Since the elements are generic types they must have implemented the Debug trait
    pub fn print(&self) {
        print!("(");
        for i in 0..self.len() {
            print!("{:?} ", self.get(i));
        }
        print!(")");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circular_vector_constructor() {
        let n = 5;
        let cv: CircularVector<f64> = CircularVector::new(n, 0.0).unwrap();
        assert_eq!(cv.len(), n); // Size is correct
        assert_eq!(cv.data.len(), n); // Internal datavector has same length
        assert_eq!(cv.tail, n - 1); // Tail pointer points to last element
    }
    #[test]
    fn circular_vector_zero_size() {
        let n = 0;
        let cv = CircularVector::<f64>::new(n, 0.0);
        assert_eq!(cv.is_err(), true); // Correctly returned Err() because size is 0
    }

    #[test]
    fn circular_index_push_and_get() {
        let n = 5;
        let mut cv: CircularVector<f64> = CircularVector::new(n, 0.0).unwrap();
        assert_eq!(cv.get(2), 0.); // Test typical element is initialized to 0.0
        let mut pop = 0.0;
        for i in 1..7 {
            pop = cv.push(i as f64);
        }
        assert_eq!(pop, 1.0); // Popped correct value
        assert_eq!(cv.get(2), 4.) // Third item is 4.0
    }
}
