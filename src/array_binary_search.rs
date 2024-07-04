use std::cmp::Ordering;

#[allow(dead_code)]
pub fn binary_search(array: Vec<i32>, target: i32) -> bool {
    if array.is_empty() {
        return false;
    }

    let mut lo: usize = 0;
    let mut hi: usize = array.len();

    while lo < hi {
        let mid: usize = lo + ((hi - lo) >> 1);
        let curr: i32 = array[mid];
        match curr.cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Not;

    #[test]
    fn edge_cases() {
        assert!(binary_search(vec![], 3).not());
        assert!(binary_search(vec![], -3).not());
        assert!(binary_search(vec![1], 3).not());
        assert!(binary_search(vec![1], -3).not());
        assert!(binary_search(vec![1], 1));
        assert!(binary_search(vec![1, 2], 2));
    }

    #[test]
    fn contains_target() {
        assert!(binary_search(vec![1, 2, 3, 4, 5], 3));
        assert!(binary_search(vec![-1, 0, 2, 3, 4, 5], 3));
        assert!(binary_search(vec![-1, 0, 2, 3, 4, 5], -1));
        assert!(binary_search(vec![-1, 0, 2, 3, 4, 5], 5));
    }

    #[test]
    fn excludes_target() {
        assert!(binary_search(vec![1, 2, 3, 4, 5], 6).not());
        assert!(binary_search(vec![1, 2, 3, 4, 5], 0).not());
        assert!(binary_search(vec![-5, -4, -3, -2, -1], 6).not());
        assert!(binary_search(vec![-5, -4, -3, -2, -1], -6).not());
    }

    #[test]
    fn large_case() {
        let array: Vec<i32> = (-100..1000).collect();
        assert!(binary_search(array.clone(), -100));
        assert!(binary_search(array.clone(), 574));
        assert!(binary_search(array.clone(), 999));
        assert!(binary_search(array.clone(), 1000).not());
        assert!(binary_search(array.clone(), -222).not());
    }
}
