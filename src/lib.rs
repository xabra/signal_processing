mod fir_filter;

pub struct CircularVector {
    size: usize,    // Number of elements
    tail: usize,    // Circular index of the most recently added element
    data: Vec<f64>, // Vector containing the data
}

// TODO: Use generics

impl CircularVector {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    /// Constructor
    pub fn new(size: usize) -> Result<Self, String> {
        if size == 0 {
            return Err("Size must be greater than zero".to_string());
        } else {
            Ok(Self {
                size: size,
                tail: size - 1, // raw index to the last element
                data: vec![0.0; size],
            })
        }
    }
    /// Get element
    /// Return vector element at index, where index 0 is the head of the vector
    pub fn get(&self, index: usize) -> f64 {
        self.data[self.circular_index(self.tail + 1 + index)]
    }

    /// Push element
    /// Push element at the end (tail) of the vector, popping off the element at the beginning (head) of the vector and returning it.
    pub fn push(&mut self, x: f64) -> f64 {
        let head_index = self.circular_index(self.tail + 1); // Head index always follows the tail inxex
        let head_value = self.data[head_index]; // Save head value to return later
        self.data[head_index] = x; // Overwrite the head with the new tail value
        self.tail = head_index; // Update the tail index to point to the new location
        head_value // Return the old (popped) head value
    }
    /// Returns the index of a CircularVector such that an index that overruns the end of the vector is mapped onto the beginning of the vector
    pub fn circular_index(&self, i: usize) -> usize {
        i % self.size
    }
    /// Debug print raw vector as it is stored internally
    pub fn print_raw(&self) {
        for i_raw in 0..self.size {
            print!("{} ", self.data[i_raw]);
        }
        print!("");
    }

    /// Debug print vector starting from the head
    pub fn print(&self) {
        print!("(");
        for i in 0..self.size {
            print!("{} ", self.get(i));
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
        let cv = CircularVector::new(n).unwrap();
        assert_eq!(cv.size, n); // Size is correct
        assert_eq!(cv.data.len(), n); // Internal datavector has same length
        assert_eq!(cv.tail, n - 1); // Tail pointer points to last element
    }
    #[test]
    fn circular_vector_zero_size() {
        let n = 0;
        let cv = CircularVector::new(n);
        assert_eq!(cv.is_err(), true); // Size is correct
    }

    #[test]
    fn circular_index_push_and_get() {
        let n = 5;
        let mut cv = CircularVector::new(n).unwrap();
        assert_eq!(cv.get(2), 0.); // Test typical element is initialized to 0.0
        let mut pop = 0.0;
        for i in 1..7 {
            pop = cv.push(i as f64);
        }
        assert_eq!(pop, 1.0); // Popped correct value
        assert_eq!(cv.get(2), 4.) // Third item is 4.0
    }
}
