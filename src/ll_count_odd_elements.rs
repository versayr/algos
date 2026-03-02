use std::collections::LinkedList;

#[allow(dead_code)]
pub fn count_odd_elements(head: LinkedList<u32>) -> u32 {
    let mut count = 0;

    for node in head.iter() {
        if node % 2 == 1 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_case() {
        let list = LinkedList::from([]);
        let count = count_odd_elements(list);
        assert_eq!(count, 0);
    }

    #[test]
    fn singleton_case() {
        let list = LinkedList::from([1]);
        let count = count_odd_elements(list);
        assert_eq!(count, 1);
    }

    #[test]
    fn large_case() {
        let list = LinkedList::from([1, 2, 3, 4, 5, 6, 7]);
        let count = count_odd_elements(list);
        assert_eq!(count, 4);

        let list = LinkedList::from([0, 0, 0, 0, 0, 0, 0]);
        let count = count_odd_elements(list);
        assert_eq!(count, 0);

        let list = LinkedList::from([1, 1, 1, 1, 1, 1, 1]);
        let count = count_odd_elements(list);
        assert_eq!(count, 7);
    }
}
