/*
   You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

   You may assume the two numbers do not contain any leading zero, except the number 0 itself.
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

#[allow(dead_code, clippy::linkedlist, clippy::needless_pass_by_value)]
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry: bool = false;
    let mut dummy_node: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: None }));
    let mut curr = dummy_node.as_mut();

    let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());

    while l1.is_some() || l2.is_some() {
        let mut sum = 0;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next.as_ref();
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next.as_ref();
        }

        if carry {
            sum += 1;
        }

        if sum > 9 {
            carry = true;
            sum -= 10;
        } else {
            carry = false;
        }

        curr.as_mut().expect("Failed to access value.").next = Some(Box::new(ListNode {
            val: sum,
            next: None,
        }));
        curr = curr.expect("Failed to access value.").next.as_mut();
    }

    if carry {
        curr.as_mut().expect("Failed to access value.").next =
            Some(Box::new(ListNode { val: 1, next: None }));
    }

    dummy_node?.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_case() {
        let l1: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: None }));
        let l2: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: None }));
        let result: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: None }));
        assert_eq!(add_two_numbers(l1, l2), result);
    }

    #[test]
    fn leetcode_cases() {
        let l1: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let result: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));
        assert_eq!(add_two_numbers(l1, l2), result);

        let l1: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode { val: 9, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let l2: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));
        let result: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode { val: 1, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(add_two_numbers(l1, l2), result);
    }
}
