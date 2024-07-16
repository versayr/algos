use::std::{rc::Rc, cell::RefCell};

#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    value: T,
    left: Option<Rc<RefCell<BinaryTree<T>>>>,
    right: Option<Rc<RefCell<BinaryTree<T>>>>,
}

#[allow(dead_code)]
impl<T: std::fmt::Debug> BinaryTree<T> {
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert_left(&mut self, value: T) {
        self.left = Some(Rc::new(RefCell::new(BinaryTree::new(value))));
    }

    pub fn insert_right(&mut self, value: T) {
        self.right = Some(Rc::new(RefCell::new(BinaryTree::new(value))));
    }

    pub fn traverse_inorder(&self) {
        if let Some(ref left) = self.left {
            left.borrow().traverse_inorder();
        }
        println!("{:?}", self.value);
        if let Some(ref right) = self.right {
            right.borrow().traverse_inorder();
        }
    }
}

#[allow(dead_code)]
pub fn dfs(root: BinaryTree<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut stack: Vec<BinaryTree<i32>> = vec![];
    stack.push(root);

    while let Some(node) = stack.pop() {
        result.push(node.value);

        if let Some(ref left) = node.left {
            stack.push(left.borrow().clone());
        }

        if let Some(ref right) = node.right {
            stack.push(right.borrow().clone());
        }

    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut root: BinaryTree<i32> = BinaryTree::new(1);
        root.insert_right(2);
        assert_eq!(dfs(root), vec![1, 2]);
        let mut root: BinaryTree<i32> = BinaryTree::new(1);
        root.insert_left(3);
        root.insert_right(2);
        assert_eq!(dfs(root), vec![1, 2, 3]);
        let mut root: BinaryTree<i32> = BinaryTree::new(1);
        root.insert_left(3);
        root.insert_right(2);
        assert_eq!(dfs(root), vec![1, 2, 3])
    }
}
//
// use std::cell::Ref;
// use::std::{rc::Rc, cell::RefCell};
// 
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     left: BinaryTree,
//     right: BinaryTree,
// }
// 
// impl Node {
//     fn new(value: i32) -> Rc<RefCell<Node>> {
//         Rc::new(RefCell::new(Node {
//             value,
//             left: BinaryTree::new(),
//             right: BinaryTree::new()
//         }))
//     }
// }
// 
// #[derive(Debug)]
// struct BinaryTree {
//     root: Option<Rc<RefCell<Node>>>
// }
// 
// impl BinaryTree {
//     fn new(root) -> Self {
//         BinaryTree { root }
//     }
// }
// 
// #[allow(dead_code)]
// pub fn dfs(root: BinaryTree) -> Vec<i32> {
//     let mut result: Vec<i32> = vec![];
//     let mut stack: Vec<BinaryTree> = vec![];
//     stack.push(root);
// 
//     while let Some(node) = stack.pop() {
//         todo!("{:?}", node);
// //         result.push(node.borrow().value);
// // 
// //         let children = node.borrow();
// // 
// //         if let Some(left) = &children.left {
// //             stack.push(Rc::clone(left));
// //         }
// // 
// //         if let Some(right) = &children.right {
// //             stack.push(Rc::clone(right));
// //         }
//     }
// 
//     result
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn test() {
// //        let root = TreeNode::new(
// //            1, 
// //            Some(TreeNode::new(2)), 
// //            None);
// //  let root = Rc::new(RefCell::new(TreeNode::new(1, None, None)));
// //         let root: BinaryTree = Some(Rc::new(RefCell::new(Node::new(
// //             1, 
// //             Some(Rc::new(RefCell::new(Node::new(2, None, None)))), 
// //             None))));
// //         assert_eq!(dfs(root), vec![1, 2])
//         let root: BinaryTree = BinaryTree::new();
//         assert_eq!(dfs(root), vec![1, 2])
//     }
// }
