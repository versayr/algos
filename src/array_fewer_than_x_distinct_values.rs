use std::collections::HashSet;

#[allow(dead_code)]
pub fn fewer_than_x_distinct_values(array: Vec<u32>, target: u32) -> bool {
    let set: HashSet<u32> = HashSet::from_iter(array.iter().cloned());
    (set.len() as u32) < target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let array = vec![];
        assert!(fewer_than_x_distinct_values(array, 3));
    }

    #[test]
    fn test_populated_array_false() {
        let array = vec![1, 2, 2, 3, 3];
        assert!(!fewer_than_x_distinct_values(array, 3));
        let array = vec![2, 2, 2, 3, 3];
        assert!(!fewer_than_x_distinct_values(array, 2));
        let array = vec![1, 2, 2, 3, 3, 4, 5, 6];
        assert!(!fewer_than_x_distinct_values(array, 3));
    }

    #[test]
    fn test_populated_array_true() {
        let array = vec![1, 2, 3, 4, 5];
        assert!(fewer_than_x_distinct_values(array, 9));
        let array = vec![1, 2, 2, 3, 3];
        assert!(fewer_than_x_distinct_values(array, 4));
        let array = vec![2, 2, 2, 3, 3];
        assert!(fewer_than_x_distinct_values(array, 3));
    }
}
