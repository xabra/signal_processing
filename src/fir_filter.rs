pub struct FIRFilter {
    weights: Vec<f64>,
    delay_line: crate::circular_vector::CircularVector<f64>,
}

// TODO:
//  Use generics
//  Initialization
//  Flush, Clear
//  Docs

impl FIRFilter {
    pub fn new(size: usize, w: Vec<f64>) -> Result<Self, String> {
        let delay_line = crate::circular_vector::CircularVector::new(size, 0.0)?;
        if w.len() != size {
            return Err("Size of weights vector must match filter size".to_string());
        } else {
            Ok(Self {
                delay_line,
                weights: w,
            })
        }
    }

    pub fn filter(&mut self, input: f64) -> f64 {
        let size = self.get_size();
        self.delay_line.push(input);
        let mut accumulator: f64 = 0.0;
        for i in 0..size {
            accumulator += self.weights[i] * self.delay_line.get(i); // Multiply & accumulate
        }
        accumulator
    }
    pub fn get_size(&self) -> usize {
        self.delay_line.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fir_filter_constructor() {
        let n = 10;
        let filter = FIRFilter::new(n, vec![1.; n]).unwrap();
        assert_eq!(filter.delay_line.len(), n);
        assert_eq!(filter.weights.len(), n);
    }
    #[test]
    fn fir_filter_constructor_zero_length() {
        let n = 0;
        assert_eq!(
            FIRFilter::new(n, vec![1., 2., 3., 4., 3., 2., 1.]).is_err(),
            true
        );
    }
    #[test]
    fn fir_filter_size_matches_weights_length() {
        let n = 10;
        assert!(FIRFilter::new(n, vec![1., 2., 3., 4., 3., 2., 1.]).is_err());
    }
    #[test]
    fn fir_filter_get_size() {
        let n = 10;
        assert_eq!(FIRFilter::new(n, vec![1.; 10]).unwrap().get_size(), n);
    }
    #[test]
    fn filter_test() {
        let size = 5;
        let n_samples = 10;
        let weights = vec![1.0 / (size as f64); size]; // Equal weights, normalized to 1.0
        let mut f = FIRFilter::new(size, weights).unwrap();
        let mut output = 0.0;
        let input = 1.0;
        println!();
        for _i in 0..n_samples {
            output = f.filter(input);
            println!("{:.2} -> {:.2}", input, output);
        }

        assert_eq!(output, 1.) // Filter's last output should be 1.0
    }
}
