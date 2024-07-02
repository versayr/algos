use std::collections::HashMap;

#[allow(dead_code)]
pub fn num_uniques(array: &[u32]) -> u32 {
    let mut duplicates: u32 = 0;
    let mut freq = HashMap::new();

    for num in array.iter() {
        freq.entry(num)
            .and_modify(|count: &mut u32| *count += 1)
            .or_insert(1);
        if freq.get(num) == Some(&2) {
            duplicates += 1;
        };
    }

    freq.len() as u32 - duplicates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let array = vec![];
        assert_eq!(num_uniques(&array), 0);
    }

    #[test]
    fn test_singleton_array() {
        let array = vec![1];
        assert_eq!(num_uniques(&array), 1);
    }

    #[test]
    fn test_long_array() {
        let array = vec![3, 1, 1, 2, 3, 1, 1, 1, 4];
        assert_eq!(num_uniques(&array), 2);
    }
}
