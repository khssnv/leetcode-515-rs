use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        // * * * 1 * * *
        // * 3 * * 2 * *
        // 5 * 3 * * 9 *
        let input = TreeNode::new(
            1,
            Some(Rc::new(RefCell::new(TreeNode::new(
                3,
                Some(Rc::new(RefCell::new(TreeNode::new(5, None, None)))),
                Some(Rc::new(RefCell::new(TreeNode::new(3, None, None)))),
            )))),
            Some(Rc::new(RefCell::new(TreeNode::new(
                2,
                None,
                Some(Rc::new(RefCell::new(TreeNode::new(9, None, None)))),
            )))),
        );
        let actual = Solution::largest_values(Some(Rc::new(RefCell::new(input))));
        let expected = vec![1, 3, 9];
        assert_eq!(actual, expected);
    }

    #[test]
    fn case_2() {
        // * 1 *
        // 2 * 3
        let input = TreeNode::new(
            1,
            Some(Rc::new(RefCell::new(TreeNode::new(2, None, None)))),
            Some(Rc::new(RefCell::new(TreeNode::new(3, None, None)))),
        );
        let actual = Solution::largest_values(Some(Rc::new(RefCell::new(input))));
        let expected = vec![1, 3];
        assert_eq!(actual, expected);
    }
}
