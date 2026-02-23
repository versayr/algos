#[allow(dead_code)]
pub fn insertion(array: &mut Vec<i32>) -> &mut Vec<i32> {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
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
        assert_eq!(insertion(&mut array), &mut sorted);

        let mut array: Vec<i32> = vec![4, 2, 3, 1];
        let mut sorted: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(insertion(&mut array), &mut sorted);

        let mut array: Vec<i32> = vec![1, 2, 3, 4];
        let mut sorted: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(insertion(&mut array), &mut sorted);
    }
}
