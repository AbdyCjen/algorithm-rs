/**
 * [114] Flatten Binary Tree to Linked List
 *
 * Given the root of a binary tree, flatten the tree into a "linked list":
 *
 *     The "linked list" should use the same TreeNode class where the right child pointer points to the next node in the list and the left child pointer is always null.
 *     The "linked list" should be in the same order as a <a href="https://en.wikipedia.org/wiki/Tree_traversal#Pre-order,_NLR" target="_blank">pre-order traversal</a> of the binary tree.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/flaten.jpg" style="width: 500px; height: 226px;" />
 * Input: root = [1,2,5,3,4,null,6]
 * Output: [1,null,2,null,3,null,4,null,5,null,6]
 *
 * Example 2:
 *
 * Input: root = []
 * Output: []
 *
 * Example 3:
 *
 * Input: root = [0]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [0, 2000].
 *     -100 <= Node.val <= 100
 *
 *  
 * Follow up: Can you flatten the tree in-place (with O(1) extra space)?
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
	pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
		fn flatten_inner(
			root: &mut Option<Rc<RefCell<TreeNode>>>,
			tail: Option<Rc<RefCell<TreeNode>>>,
		) {
			if let Some(root) = root {
				let mut root = root.borrow_mut();
				let mut right = root.right.take();
				flatten_inner(&mut right, tail);
				flatten_inner(&mut root.left, right);
				root.right = root.left.take();
			} else {
				*root = tail;
			}
		}
		if root.is_some() {
			flatten_inner(root, None);
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_114() {
		let mut tr = tree![1, 2, 5, 3, 4, null, 6];
		Solution::flatten(&mut tr);
		assert_eq!(tr, tree![1, null, 2, null, 3, null, 4, null, 5, null, 6]);

		let mut tr = None;
		Solution::flatten(&mut tr);
		assert_eq!(tr, None);
	}
}
