#[allow(dead_code)]
pub fn generate_superset(array: &[u32]) -> Vec<Vec<u32>> {
    let mut output: Vec<Vec<u32>> = vec![];
    let max = u32::pow(
        2,
        u32::try_from(array.len()).expect("Value may be truncated due to 64-bit wide pointers."),
    );

    for mask in 0..max {
        output.push(
            array
                .iter()
                .enumerate()
                .filter(|&(idx, _)| 0 < (mask & (1 << idx)))
                .map(|(_, element)| element)
                .copied()
                .collect(),
        );
    };

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(generate_superset(&vec![]), vec![vec![]])
    }

    #[test]
    fn test_small() {
        let array = vec![1, 2];
        let result: Vec<Vec<u32>> = vec![vec![], vec![1], vec![2], vec![1, 2]];
        assert_eq!(generate_superset(&array), result)
    }

    #[test]
    fn test_large() {
        let array = vec![0, 1, 2, 3, 4, 5];
        let result: Vec<Vec<u32>> = vec![
            vec![],
            vec![0],
            vec![1],
            vec![0, 1],
            vec![2],
            vec![0, 2],
            vec![1, 2],
            vec![0, 1, 2],
            vec![3],
            vec![0, 3],
            vec![1, 3],
            vec![0, 1, 3],
            vec![2, 3],
            vec![0, 2, 3],
            vec![1, 2, 3],
            vec![0, 1, 2, 3],
            vec![4],
            vec![0, 4],
            vec![1, 4],
            vec![0, 1, 4],
            vec![2, 4],
            vec![0, 2, 4],
            vec![1, 2, 4],
            vec![0, 1, 2, 4],
            vec![3, 4],
            vec![0, 3, 4],
            vec![1, 3, 4],
            vec![0, 1, 3, 4],
            vec![2, 3, 4],
            vec![0, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![5],
            vec![0, 5],
            vec![1, 5],
            vec![0, 1, 5],
            vec![2, 5],
            vec![0, 2, 5],
            vec![1, 2, 5],
            vec![0, 1, 2, 5],
            vec![3, 5],
            vec![0, 3, 5],
            vec![1, 3, 5],
            vec![0, 1, 3, 5],
            vec![2, 3, 5],
            vec![0, 2, 3, 5],
            vec![1, 2, 3, 5],
            vec![0, 1, 2, 3, 5],
            vec![4, 5],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![0, 1, 4, 5],
            vec![2, 4, 5],
            vec![0, 2, 4, 5],
            vec![1, 2, 4, 5],
            vec![0, 1, 2, 4, 5],
            vec![3, 4, 5],
            vec![0, 3, 4, 5],
            vec![1, 3, 4, 5],
            vec![0, 1, 3, 4, 5],
            vec![2, 3, 4, 5],
            vec![0, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![0, 1, 2, 3, 4, 5],
        ];
        assert_eq!(generate_superset(&array), result)
    }
}
