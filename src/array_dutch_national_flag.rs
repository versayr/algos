#[allow(dead_code)]
pub fn dutch_national_flag(mut array: Vec<u32>) -> Vec<u32> {
    if array.is_empty() {
        return array;
    };
    let mut lo: usize = 0;
    let mut hi: usize = array.len() - 1;
    let mut curr: usize = 0;

    while curr <= hi {
        if array[curr] == 0 {
            array.swap(lo, curr);
            curr += 1;
            lo += 1;
        } else if array[curr] == 2 {
            array.swap(hi, curr);
            hi -= 1;
        } else if array[curr] == 1 {
            curr += 1;
        };
    }

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let array = vec![];
        assert_eq!(dutch_national_flag(array), []);
    }

    #[test]
    fn test_sorted_array() {
        let array = vec![0, 0, 1, 1, 2, 2];
        assert_eq!(dutch_national_flag(array), [0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_full_array() {
        let array = vec![2, 1, 1, 0];
        assert_eq!(dutch_national_flag(array), [0, 1, 1, 2]);
        let array = vec![0, 2, 1, 0, 1, 2];
        assert_eq!(dutch_national_flag(array), [0, 0, 1, 1, 2, 2]);
    }
}
