use std::collections::LinkedList;

#[allow(dead_code, clippy::linkedlist)]
pub fn partition_list(mut list: LinkedList<i32>, parts: usize) -> Vec<LinkedList<i32>> {
    let mut result = Vec::with_capacity(parts);
    let mut remainder = list.len() % parts;
    let base_size = list.len() / parts;

    for _ in 0..parts {
        let mut segment = LinkedList::new();
        for _ in 0..base_size {
            if let Some(node) = list.pop_front() {
                segment.push_back(node);
            }
        }

        if remainder > 0 && let Some(node) = list.pop_front() {
            segment.push_back(node);
            remainder = remainder.saturating_sub(1);
        }

        result.push(segment);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn result_to_string(list: Vec<LinkedList<i32>>) -> String {
        let mut result: Vec<String> = vec![];

        for segment in list.iter() {
            let s: String = segment.iter().fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push(',');
                }
                acc.push_str(&s.to_string());
                acc
            });
            result.push(format!("[{}]", s));
        }

        format!("[{}]", result.join(","))
    }

    #[test]
    fn empty_case() {
        let list: LinkedList<i32> = LinkedList::from([]);
        let result = partition_list(list, 1);
        let goal = String::from("[[]]");
        assert_eq!(result_to_string(result), goal);
    }

    #[test]
    fn singleton_case() {
        let list = LinkedList::from([1]);
        let result = partition_list(list, 2);
        let goal = String::from("[[1],[]]");
        assert_eq!(result_to_string(result), goal);
    }

    #[test]
    fn large_case() {
        let list = LinkedList::from([1, 2, 3, 4, 5]);
        let result = partition_list(list, 2);
        let goal = String::from("[[1,2,3],[4,5]]");
        assert_eq!(result_to_string(result), goal);

        let list = LinkedList::from([1, 2, 3, 4, 5]);
        let result = partition_list(list, 10);
        let goal = String::from("[[1],[2],[3],[4],[5],[],[],[],[],[]]");
        assert_eq!(result_to_string(result), goal);

        let list = LinkedList::from([0, 0, 0, 0, 0, 0, 0]);
        let result = partition_list(list, 3);
        let goal = String::from("[[0,0,0],[0,0],[0,0]]");
        assert_eq!(result_to_string(result), goal);

        let list = LinkedList::from([1, 1, 1, 1, 1, 1, 1]);
        let result = partition_list(list, 7);
        let goal = String::from("[[1],[1],[1],[1],[1],[1],[1]]");
        assert_eq!(result_to_string(result), goal);
    }
}
