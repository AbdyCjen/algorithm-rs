/**
 * [226] Invert Binary Tree
 *
 * Invert a binary tree.
 *
 * Example:
 *
 * Input:
 *
 *
 *      4
 *    /   \
 *   2     7
 *  / \   / \
 * 1   3 6   9
 *
 * Output:
 *
 *
 *      4
 *    /   \
 *   7     2
 *  / \   / \
 * 9   6 3   1
 *
 * Trivia:<br />
 * This problem was inspired by <a href="https://twitter.com/mxcl/status/608682016205344768" target="_blank">this original tweet</a> by <a href="https://twitter.com/mxcl" target="_blank">Max Howell</a>:
 *
 * <blockquote>Google: 90% of our engineers use the software you wrote (Homebrew), but you can&rsquo;t invert a binary tree on a whiteboard so f*** off.</blockquote>
 *
 */
pub struct Solution {}
use super::util::tree::TreeNode;

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::{cell::RefCell, rc::Rc};
#[allow(dead_code)]
impl Solution {
	pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		if let Some(root) = root {
			Self::invert(&mut *root.borrow_mut());
			Some(root)
		} else {
			root
		}
	}
	fn invert(root: &mut TreeNode) {
		std::mem::swap(&mut root.left, &mut root.right);
		if let Some(o) = root.left.as_mut() {
			Self::invert(&mut *o.borrow_mut())
		}
		if let Some(o) = root.right.as_mut() {
			Self::invert(&mut *o.borrow_mut())
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_226() {
		assert_eq!(
			Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9]),
			tree![4, 7, 2, 9, 6, 3, 1]
		);
	}
}
