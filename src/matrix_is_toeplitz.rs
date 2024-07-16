#[allow(dead_code)]
pub fn is_toeplitz(matrix: Vec<Vec<i32>>) -> bool {
    for (x, row) in matrix.iter().enumerate().take(matrix.len() - 1) {
        for (y, element) in row.iter().enumerate().take(matrix.len() - 1) {
            if *element != matrix[x + 1][y + 1] {
                return false;
            }
        }
    }

    true
}

#[allow(dead_code)]
pub fn is_toeplitz_bi(matrix: Vec<Vec<i32>>) -> bool {
    let mut right: bool = true;
    let mut left: bool = true;

    for (x, row) in matrix.iter().enumerate().take(matrix.len() - 1) {
        for (y, element) in row.iter().enumerate().take(matrix.len() - 1) {
            if *element != matrix[x + 1][y + 1] {
                right = false;
                break;
            }
        }
    }

    for (x, row) in matrix.iter().enumerate().take(matrix.len() - 1) {
        for (y, element) in row.iter().enumerate().skip(1) {
            if *element != matrix[x + 1][y - 1] {
                left = false;
                break;
            }
        }
    }

    right || left
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Not;

    #[test]
    fn unidirectional() {
        let matrix = vec![vec![1, 2], vec![2, 2]];
        assert!(is_toeplitz(matrix).not());
        let matrix = vec![vec![1, 2], vec![2, 1]];
        assert!(is_toeplitz(matrix));
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![6, 5, 1, 2],
            vec![7, 6, 5, 1],
        ];
        assert!(is_toeplitz(matrix));
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![6, 5, 1, 2],
            vec![7, 6, 5, 9],
        ];
        assert!(is_toeplitz(matrix).not());
        let matrix = vec![
            vec![1, 2, 6, 4],
            vec![2, 6, 4, 3],
            vec![6, 4, 3, 5],
            vec![4, 3, 5, 9],
        ];
        assert!(is_toeplitz(matrix).not());
    }

    #[test]
    fn bidirectional() {
        let matrix = vec![vec![1, 2], vec![2, 2]];
        assert!(is_toeplitz_bi(matrix));
        let matrix = vec![vec![1, 2], vec![2, 1]];
        assert!(is_toeplitz_bi(matrix));
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![6, 5, 1, 2],
            vec![7, 6, 5, 1],
        ];
        assert!(is_toeplitz_bi(matrix));
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![6, 5, 1, 2],
            vec![7, 6, 5, 9],
        ];
        assert!(is_toeplitz_bi(matrix).not());
        let matrix = vec![
            vec![1, 2, 6, 4],
            vec![2, 6, 4, 3],
            vec![6, 4, 3, 5],
            vec![4, 3, 5, 9],
        ];
        assert!(is_toeplitz_bi(matrix));
    }
}
//     OLD
//     for i in 0..matrix.len() - 1 {
//         for j in 0..matrix[0].len() - 1 {
//             if matrix[i][j] != matrix[i + 1][j + 1] {
//                 right = false;
//             }
//         }
//     }

//     for i in 0..matrix.len() - 1 {
//         for j in 1..matrix[0].len() - 1 {
//             if matrix[i][j] != matrix[i + 1][j - 1] {
//                 left = false;
//             }
//         }
//     }
//
//     for (x, row) in matrix.iter().enumerate() {
//         if let Some(next_row) = matrix.get(x + 1) {
//             for (y, element) in row.iter().enumerate() {
//                 if let Some(next_element) = next_row.get(y + 1) {
//                     if *element != *next_element {
//                         right = false;
//                         break;
//                     }
//                 } else {
//                     break;
//                 };
//             }
//         } else {
//             break;
//         }
//     }
