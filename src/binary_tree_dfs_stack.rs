use::std::{rc::Rc, cell::RefCell};

type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub struct TreeNode {
    pub value: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}

impl TreeNode {
    pub fn new(value: i32, left: Option<TreeNodeRef>, right: Option<TreeNodeRef>) -> Self {
        Self { 
            value, 
            left, 
            right, 
        } 
    }
}

#[allow(dead_code)]
pub fn dfs(root: TreeNodeRef) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut stack: Vec<TreeNodeRef> = vec![];
    stack.push(root);

    while let Some(node) = stack.pop() {
        result.push(node.borrow().value);

        let children = node.borrow();

        if let Some(left) = &children.left {
            stack.push(Rc::clone(left));
        }

        if let Some(right) = &children.right {
            stack.push(Rc::clone(right));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
//        let root = TreeNode::new(
//            1, 
//            Some(TreeNode::new(2)), 
//            None);
        let root = Rc::new(RefCell::new(TreeNode::new(1, None, None)));
        assert_eq!(dfs(root), vec![1])
    }
}
