#[allow(dead_code)]
pub fn max_subarray_sum(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut sum: i32 = *nums.first().expect("Array must not be empty.");
    let mut max: i32 = *nums.first().expect("Array must not be empty.");

    for &num in nums.iter().skip(1) {
        sum = sum.max(0) + num;
        max = max.max(sum);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_subarray_sum(&[1]), 1);
        assert_eq!(max_subarray_sum(&[5, 4, -1, 7, 8]), 23);
        assert_eq!(max_subarray_sum(&[]), 0);
    }
}
