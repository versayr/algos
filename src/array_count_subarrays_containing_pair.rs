use std::collections::HashMap;

#[allow(dead_code)]
pub fn subarrays_sum_to_k(array: &Vec<u32>, subarray_length: u32, sum: u32) -> u32 {
    let subarray_length = subarray_length as usize;

    let mut result: u32 = 0;
    let mut freq = HashMap::new();
    let mut pairs_in_window: u32 = 0;

    for i in 0..array.len() {
        let curr = &array[i];
        if freq.contains_key(&(sum - curr)) {
            pairs_in_window += 1;
        };

        if i >= subarray_length {
            let last = &array[i - subarray_length];

            if freq.get(last) == Some(&1) {
                freq.remove(last);
            } else {
                freq.insert(last, freq.get(last).unwrap() - 1);
            };

            if freq.contains_key(&(sum - last)) {
                pairs_in_window -= 1;
            };
        };

        if i >= subarray_length - 1 && pairs_in_window > 0 {
            result += 1;
        };

        match freq.get(curr) {
            Some(count) => {
                freq.insert(curr, count + 1);
            },
            None => {
                freq.insert(curr, 1);
            }
        };
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varied_array() {
        let array = vec![2, 4, 7, 5, 3, 5, 8, 5, 1, 7];
        assert_eq!(subarrays_sum_to_k(&array, 4, 10), 5);
    }

    #[test]
    fn test_uniform_array() {
        let array = vec![2, 2, 2, 2, 2];
        assert_eq!(subarrays_sum_to_k(&array, 3, 4), 3);
        assert_eq!(subarrays_sum_to_k(&array, 3, 6), 0);
    }
}
