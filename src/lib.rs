pub struct CircularVector {
    size: usize,    // Number of elements
    tail: usize,    // Circular index of the most recently added element
    data: Vec<f64>, // Vector containing the data
}
impl CircularVector {
    /// Constructor
    pub fn new(size: usize) -> Self {
        // Should check for size == 0 here. Error
        Self {
            size: size,
            tail: size - 1, // raw index to the last element
            data: vec![0.0; size],
        }
    }
    /// Return vector element at index, where index 0 is the head of the vector
    pub fn get(&self, index: usize) -> f64 {
        self.data[self.circular_index(self.tail + 1 + index)]
    }

    /// Push an element at the end (tail) of the vector, popping off the element at the beginning (head) of the vector and returning it.
    pub fn push(&mut self, x: f64) -> f64 {
        let head_index = self.circular_index(self.tail + 1); // Head index always follows the tail inxex
        let head_value = self.data[head_index]; // Save head value to return later
        self.data[head_index] = x; // Overwrite the head with the new tail value
        self.tail = head_index; // Update the tail index to point to the new location
        head_value // Return the old (popped) head value
    }
    /// Returns the index of a circular vector or Vec such that an index that overruns the end of the vector is mapped onto the beginning of the vector
    pub fn circular_index(&self, i: usize) -> usize {
        i % self.size
    }
    /// Debug print raw vector as it is stored internally
    pub fn print_raw(&self) {
        for i_raw in 0..self.size {
            print!("{} ", self.data[i_raw]);
        }
        println!("");
    }

    /// Debug print vector starting from the head
    pub fn print(&self) {
        print!("(");
        for i in 0..self.size {
            print!("{} ", self.get(i));
        }
        println!(")");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circular_index_positive_index() {
        assert_eq!(1, 1);
    }

    #[test]
    fn circular_index_zero_index() {
        assert_eq!(0, 0);
    }
}
