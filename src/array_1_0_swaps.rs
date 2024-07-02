use std::cmp::min;

#[allow(dead_code)]
pub fn min_swaps(array: &[u32]) -> u32 {
    let mut zero_swaps = 0;
    let mut one_swaps = 0;
    let mut zero_idx = 0;
    let mut one_idx = 0;

    for (i, item) in array.iter().enumerate() {
        if *item == 0 {
            zero_swaps += i - zero_idx;
            zero_idx += 1;
        } else {
            one_swaps += i - one_idx;
            one_idx += 1;
        }
    }

    min(zero_swaps as u32, one_swaps as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_size_2() {
        let array = vec![0, 1];
        assert_eq!(min_swaps(&array), 0);
    }

    #[test]
    fn test_array_size_3() {
        let array = vec![0, 1, 0];
        assert_eq!(min_swaps(&array), 1);
    }

    #[test]
    fn test_array_size_5() {
        let array = vec![1, 0, 1, 1, 0];
        assert_eq!(min_swaps(&array), 2);
    }

    #[test]
    fn test_array_size_5_again() {
        let array = vec![0, 1, 1, 1, 0];
        assert_eq!(min_swaps(&array), 3);
    }

    #[test]
    fn test_array_size_12() {
        let array = vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(min_swaps(&array), 4);
    }

    #[test]
    fn test_array_largest() {
        let array = vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0];
        assert_eq!(min_swaps(&array), 16);

    }
}
