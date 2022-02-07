/// Returns the index of a circular array or Vec such that
/// an index off either end of the array is mapped onto the buffer
/// Negative indices are handled correctly
pub fn circular_index(i: i32, n: i32) -> i32 {
    let mut j = i % n;
    if j < 0 {
        j += n
    };
    j
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circular_index_positive_index() {
        let result = circular_index(6, 5);
        assert_eq!(result, 1);
    }
    #[test]
    fn circular_index_negative_index() {
        let result = circular_index(-6, 5);
        assert_eq!(result, 4);
    }
    #[test]
    fn circular_index_zero_index() {
        let result = circular_index(0, 5);
        assert_eq!(result, 0);
    }
}
