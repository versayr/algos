use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(array: Vec<i32>, target: i32) -> Result<Vec<usize>, &'static str> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (idx, value) in array.iter().enumerate() {
        match map.get(&(target - value)) {
            Some(result) => return Ok(vec![*result, idx]),
            None => map.insert(*value, idx),
        };
    }

    Err("No valid results found.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_cases() {
        assert_eq!(two_sum(vec![], 4), Err("No valid results found."))
    }

    #[test]
    fn regular_cases() {
        assert_eq!(two_sum(vec![0, 1, 2, 3, 4], 4), Ok(vec![1, 3]))
    }
}
