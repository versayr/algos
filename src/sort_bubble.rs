#[allow(dead_code)]
pub fn bubble(array: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut swapped: bool = true;

    while swapped {
        swapped = false;
        for i in 0..array.len() - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }
    }

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let mut array: Vec<i32> = vec![1];
        let mut sorted: Vec<i32> = vec![1];
        assert_eq!(bubble(&mut array), &mut sorted);

        let mut array: Vec<i32> = vec![4, 2, 3, 1];
        let mut sorted: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(bubble(&mut array), &mut sorted);

        let mut array: Vec<i32> = vec![1, 2, 3, 4];
        let mut sorted: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(bubble(&mut array), &mut sorted);
    }
}
